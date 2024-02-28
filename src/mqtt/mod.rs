pub mod data;

use crate::data::common::{Broker, Protocol};
use crate::data::AppEvent;
use crate::mqtt::data::{MqttPublicInput, MqttSubscribeInput};

use anyhow::{bail, Result};

use log::{debug, error, info};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

use crate::data::db::{ClientTls, Credentials, Tls};
use for_mqtt_client::protocol::packet::Publish;
use for_mqtt_client::protocol::MqttOptions;
use for_mqtt_client::tls::{CertificateFile, PrivateKeyFile, TlsConfig};
use for_mqtt_client::MqttEvent;
pub use for_mqtt_client::{Client, QoS, QoSWithPacketId};
use tauri::{AppHandle, Manager};

pub async fn init_connect(broker: Broker, tx: AppHandle) -> Result<Client> {
    let broker_db = broker.data.clone();
    let port = broker_db.port;
    let mut mqttoptions =
        MqttOptions::new(broker_db.client_id.clone(), broker_db.addr.as_str(), port)?;
    if let Credentials::Credentials {
        user_name,
        password,
    } = broker_db.credentials
    {
        mqttoptions = mqttoptions.set_credentials(user_name, password);
    }
    let some = serde_json::from_str(broker_db.params.as_str())?;
    mqttoptions = update_tls_option(update_option(mqttoptions.clone(), some), broker.clone());
    if broker_db.auto_connect {
        mqttoptions = mqttoptions.auto_reconnect();
    }
    debug!("{:?}", mqttoptions);
    let (client, mut eventloop) = match broker_db.protocol {
        Protocol::V4 => mqttoptions.connect_to_v4().await?,
        Protocol::V5 => mqttoptions.connect_to_v5().await?,
    };
    let id = broker_db.id;
    tokio::spawn(async move {
        let tx = &tx;
        while let Ok(event) = eventloop.recv().await {
            match event.as_ref() {
                MqttEvent::ConnectSuccess(retain) => {
                    send_event(
                        tx,
                        AppEvent::ConnectAckSuccess {
                            broker_id: id,
                            retain: *retain,
                        },
                    );
                }
                MqttEvent::ConnectFail(err) => {
                    send_event(tx, AppEvent::ConnectAckFail(id, format!("{:?}", err)));
                }
                MqttEvent::PublishSuccess(packet_id) => {
                    send_event(tx, AppEvent::PubAck(id, *packet_id));
                }
                MqttEvent::SubscribeAck(packet) => {
                    send_event(
                        tx,
                        AppEvent::SubAck {
                            broker_id: id,
                            ack: packet.clone(),
                        },
                    );
                }
                MqttEvent::UnsubscribeAck(packet) => {
                    send_event(tx, AppEvent::UnSubAck(id, packet.clone()));
                }
                MqttEvent::Publish(msg) => {
                    let Publish {
                        qos,
                        topic,
                        payload,
                        ..
                    } = msg;
                    debug!("recv publish: {} payload len = {}", topic, payload.len());
                    send_event(
                        tx,
                        AppEvent::ReceivePublic {
                            broker_id: id,
                            topic: topic.clone(),
                            payload: payload.clone(),
                            qos: (*qos).into(),
                        },
                    );
                }
                MqttEvent::PublishFail(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::SubscribeFail(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::ConnectedErr(reason) => {
                    error!("{}", reason);
                    send_event(tx, AppEvent::ConnectedErr(id, reason.clone()));
                }
                MqttEvent::UnsubscribeFail(reason) => {
                    error!("{}", reason);
                }
                MqttEvent::Disconnected => {
                    send_event(tx, AppEvent::Disconnect(id));
                    info!("Disconnected");
                }
            }
        }
    });
    Ok(client)
}

fn send_event(tx: &AppHandle, event: AppEvent) {
    let Some((event, event_data)) = event.event() else {
        return;
    };
    if if let Some(event_data) = event_data {
        tx.emit_all(event, event_data)
    } else {
        tx.emit_all(event, ())
    }
    .is_err()
    {
        error!("mqtt-loop fail to send event!");
    }
}

pub async fn mqtt_subscribe(
    index: usize,
    input: MqttSubscribeInput,
    clients: &HashMap<usize, Client>,
) -> Result<()> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    debug!("{:?}", input);
    Ok(client
        .to_subscribe_with_trace_id(input.topic, input.qos.into(), input.trace_id)
        .await?)
}

pub async fn to_unsubscribe(
    index: usize,
    topic: String,
    clients: &HashMap<usize, Client>,
) -> Result<u32> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client.unsubscribe(topic).await?)
}

pub async fn mqtt_public(
    index: usize,
    input: MqttPublicInput,
    clients: &HashMap<usize, Client>,
) -> Result<()> {
    let Some(client) = clients.get(&index) else {
        bail!("can't get mqtt client: {}", index);
    };
    Ok(client
        .publish_with_trace_id(
            input.topic,
            input.qos.into(),
            input.msg,
            input.retain,
            input.trace_id,
        )
        .await?)
}

fn update_option(option: MqttOptions, some: SomeMqttOption) -> MqttOptions {
    let SomeMqttOption {
        keep_alive,
        clean_session,
        max_incoming_packet_size,
        max_outgoing_packet_size,
        inflight: _,
        conn_timeout: _,
    } = some;
    option
        .set_clean_session(clean_session)
        .set_max_packet_size(max_incoming_packet_size, max_outgoing_packet_size)
        .set_keep_alive(keep_alive)
}

#[derive(Serialize, Deserialize, Debug)]
pub struct SomeMqttOption {
    // seconds
    keep_alive: u16,
    clean_session: bool,
    max_incoming_packet_size: usize,
    max_outgoing_packet_size: usize,
    inflight: u16,
    // seconds
    conn_timeout: u64,
}

impl Default for SomeMqttOption {
    fn default() -> Self {
        Self {
            keep_alive: 60,
            clean_session: true,
            max_incoming_packet_size: 10 * 1024,
            max_outgoing_packet_size: 10 * 1024,
            inflight: 100,
            conn_timeout: 5,
        }
    }
}

fn update_tls_option(option: MqttOptions, value: Broker) -> MqttOptions {
    let tls_config = match value.data.tls {
        Tls::None => return option,
        Tls::Ca => TlsConfig::default(),
        Tls::Insecurity => TlsConfig::default().insecurity(),
        Tls::SelfSigned { self_signed_ca } => {
            TlsConfig::default().set_server_ca_pem_file(self_signed_ca.as_str().into())
        }
    };
    match value.data.client_tls {
        ClientTls::None => option.set_tls(tls_config),
        ClientTls::Verify {
            certificate,
            private_key,
        } => {
            let tls_config = tls_config.verify_client(
                CertificateFile::Pem(certificate.into()),
                PrivateKeyFile::Rsa(private_key.into()),
            );
            option.set_tls(tls_config)
        }
    }
}

#[cfg(test)]
mod test {
    use crate::mqtt::SomeMqttOption;

    #[test]
    fn test_option() {
        let option = SomeMqttOption::default();
        println!("{}", serde_json::to_string(&option).unwrap());

        let option_str = r#"{
	"keep_alive": 60,
	"clean_session": true,
	"max_incoming_packet_size": 10240,
	"max_outgoing_packet_size": 10240,
	"inflight": 100,
	"conn_timeout": 5
}
        "#;
        let option: SomeMqttOption = serde_json::from_str(option_str).unwrap();
        println!("{:?}", option);
    }
}
