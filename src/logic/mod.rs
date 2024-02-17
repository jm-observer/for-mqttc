

use crate::mqtt::{init_connect, mqtt_public, mqtt_subscribe};
// use crate::ui::tabs::init_brokers_tabs;
// use crate::data::click_ty::ClickTy;
use crate::data::common::{Broker, SubscribeTopic};
use crate::mqtt::data::MqttPublicInput;
use crate::mqtt::Client;



use anyhow::Result;





use log::{error};
use std::collections::HashMap;






use tauri::AppHandle;



pub async fn connect(mqtt_clients: &mut HashMap<usize, Client>, tx: AppHandle, broker: Broker) {
    let id = broker.data.id;
    if let Some(old_client) = mqtt_clients.remove(&id) {
        if let Err(err) = old_client.disconnect().await {
            error!("diconnect fail: {:?}", err);
        };
    };
    match init_connect(broker, tx).await {
        Ok(client) => {
            mqtt_clients.insert(id, client);
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

pub async fn to_subscribe(mqtt_clients: &HashMap<usize, Client>, input: SubscribeTopic) {
    match mqtt_subscribe(input.broker_id, input.clone().into(), mqtt_clients).await {
        Ok(()) => {}
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

pub async fn to_publish(
    mqtt_clients: &HashMap<usize, Client>,
    publish: MqttPublicInput,
) -> Result<()> {
    mqtt_public(publish.broker_id, publish, mqtt_clients).await?;
    Ok(())
}

pub async fn to_disconnect(mqtt_clients: &mut HashMap<usize, Client>, id: usize) -> Result<()> {
    if let Some(client) = mqtt_clients.remove(&id) {
        client.disconnect().await?;
    }
    Ok(())
}
