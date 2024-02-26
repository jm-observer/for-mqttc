mod error;
pub mod view;

use crate::command::error::Error;
use crate::command::view::{BrokerList, BrokerView, TlsView, ViewConfig};
use crate::data::common::{PublishHis, PublishInput, SubscribeHis, SubscribeInput, SubscribeTopic};
use crate::data::db::BrokerDB;
use crate::data::hierarchy::App;

use crate::logic::{connect, to_disconnect, to_publish, to_subscribe};
use crate::mqtt::data::MqttPublicInput;

use log::debug;

use crate::config::Config;
use crate::mqtt::to_unsubscribe;
use std::mem::swap;
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::{command, State};
use tokio::fs;
use tokio::sync::RwLock;

type ArcApp = RwLock<App>;
type Result<T> = std::result::Result<T, Error>;

#[command]
pub async fn broker_list(state: State<'_, ArcApp>) -> Result<String> {
    debug!("broker_list");
    let app = state.read().await;
    let brokers = app.brokers.clone().into_iter();
    let brokers: Vec<BrokerView> = brokers.map(|x| x.data).map(BrokerView::from).collect();
    let rs = BrokerList { brokers };
    let rs = serde_json::to_string(&rs)?;
    Ok(rs)
}

#[command]
pub async fn subscribe(datas: SubscribeInput, state: State<'_, ArcApp>) -> Result<()> {
    debug!("subscribe: {:?}", datas);
    let mut app = state.write().await;
    app.update_subscribe_his(datas.clone())?;
    to_subscribe(&app.mqtt_clients, SubscribeTopic::from(datas)).await;
    Ok(())
}

#[command]
pub async fn unsubscribe(broker_id: usize, topic: String, state: State<'_, ArcApp>) -> Result<u32> {
    let app = state.read().await;
    let tarce_id = to_unsubscribe(broker_id, topic, &app.mqtt_clients).await?;
    Ok(tarce_id)
}

#[command]
pub async fn publish(datas: PublishInput, state: State<'_, ArcApp>) -> Result<()> {
    debug!("publish: {:?}", datas);
    let mut app = state.write().await;
    app.update_publish_his(datas.clone())?;
    to_publish(&app.mqtt_clients, MqttPublicInput::try_from(datas)?).await?;
    Ok(())
}

#[command]
pub async fn publish_his(broker_id: usize, state: State<'_, ArcApp>) -> Result<Vec<PublishHis>> {
    let app = state.read().await;
    let mut rs = app
        .brokers
        .iter()
        .find(|x| x.data.id == broker_id)
        .map(|x| x.data.publish_his.clone())
        .unwrap_or_default();
    rs.reverse();
    Ok(rs)
}

#[command]
pub async fn delete_publish_his(
    broker_id: usize,
    his: PublishHis,
    state: State<'_, ArcApp>,
) -> Result<()> {
    let mut app = state.write().await;
    let Some(broker) = app.brokers.iter_mut().find(|x| x.data.id == broker_id) else {
        return Error::init_rs(format!("not found broker {}", broker_id));
    };

    let Some(index) = broker.data.publish_his.iter().enumerate().find_map(|x| {
        if x.1 == &his {
            Some(x.0)
        } else {
            None
        }
    }) else {
        return Error::init_rs("not found publish his");
    };
    broker.data.publish_his.remove(index);
    broker.db.save_broker(&broker.data)?;
    Ok(())
}

#[command]
pub async fn delete_subscribe_his(
    broker_id: usize,
    his: SubscribeHis,
    state: State<'_, ArcApp>,
) -> Result<()> {
    let mut app = state.write().await;
    let Some(broker) = app.brokers.iter_mut().find(|x| x.data.id == broker_id) else {
        return Error::init_rs(format!("not found broker {}", broker_id));
    };
    let Some(index) = broker.data.subscribe_his.iter().enumerate().find_map(|x| {
        if x.1 == &his {
            Some(x.0)
        } else {
            None
        }
    }) else {
        return Error::init_rs("not found publish his");
    };
    broker.data.subscribe_his.remove(index);
    broker.db.save_broker(&broker.data)?;
    Ok(())
}
#[command]
pub async fn subscribe_his(
    broker_id: usize,
    state: State<'_, ArcApp>,
) -> Result<Vec<SubscribeHis>> {
    let app = state.read().await;
    let mut rs = app
        .brokers
        .iter()
        .find(|x| x.data.id == broker_id)
        .map(|x| x.data.subscribe_his.clone())
        .unwrap_or_default();
    rs.reverse();
    Ok(rs)
}

#[command]
pub async fn disconnect(id: usize, state: State<'_, ArcApp>) -> Result<()> {
    debug!("disconnect: {}", id);
    let mut app = state.write().await;
    to_disconnect(&mut app.mqtt_clients, id).await?;
    Ok(())
}

#[command]
pub async fn delete_broker(id: usize, state: State<'_, ArcApp>) -> Result<()> {
    debug!("disconnect: {}", id);
    let mut app = state.write().await;
    to_disconnect(&mut app.mqtt_clients, id).await?;
    let mut brokers = Vec::with_capacity(0);
    swap(&mut brokers, &mut app.brokers);
    app.brokers = brokers.into_iter().filter(|x| x.data.id != id).collect();
    app.db.delete_broker(id)?;
    Ok(())
}

#[command]
pub async fn connect_to_broker(
    id: usize,
    state: State<'_, ArcApp>,
    app_handle: AppHandle,
) -> Result<()> {
    debug!("connect_to_broker: {id}");
    let mut app = state.write().await;
    let Some(broker) = app.brokers.iter().find_map(|x| {
        if x.data.id == id {
            Some(x.clone())
        } else {
            None
        }
    }) else {
        return Error::init_rs(format!("not found broker {}", id));
    };
    connect(&mut app.mqtt_clients, app_handle, broker).await;
    // app.tx.send(AppEvent::ToConnect(broker)).map_err(|_| {
    //     error!("error");
    //     "send ToConnect event fail".to_string()
    // })?;
    Ok(())
}

#[command]
pub async fn update_or_new_broker(
    mut broker: BrokerView,
    state: State<'_, ArcApp>,
) -> Result<usize> {
    debug!("{:?}", broker);
    let mut app = state.write().await;
    if broker.id == 0 {
        broker.id = app.db.next_broker_id();
    }
    if let TlsView::SelfSigned = broker.tls {
        let broker_path = app.home_path.join(broker.id.to_string());
        let ca_path: PathBuf = broker.self_signed_ca.into();
        let (Some(parent), Some(file_name)) = (ca_path.parent(), ca_path.file_name()) else {
            return Error::init_rs("self signed ca copied fail");
        };
        let broker_ca_path = broker_path.join(file_name);
        if parent != broker_path.as_path() {
            fs::create_dir_all(broker_path).await?;
            // copy to home_path/id
            fs::copy(ca_path, broker_ca_path.as_path()).await?;
        }
        broker.self_signed_ca = broker_ca_path
            .to_str()
            .ok_or(Error::init("self signed ca copied fail"))?
            .to_string();
    }
    let id = broker.id;
    let data: BrokerDB = broker.into();
    app.save_broker(data)?;
    Ok(id)
}

#[command]
pub async fn loading(state: State<'_, ArcApp>) -> Result<ViewConfig> {
    let mut app = state.write().await;
    let clients = std::mem::take(&mut app.mqtt_clients);
    for client in clients.values() {
        let _ = client.disconnect().await;
    }
    Ok(ViewConfig::init(&app, &Config::init(app.home_path.clone())))
}
