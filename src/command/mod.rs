mod error;
pub mod view;

use crate::command::error::Error;
use crate::command::view::{BrokerList, BrokerView, TlsView};
use crate::data::common::{PublishInput, SubscribeInput, SubscribeTopic};
use crate::data::db::BrokerDB;
use crate::data::hierarchy::App;
use crate::data::AppEvent;
use crate::logic::{connect, to_disconnect, to_publish, to_subscribe};
use crate::mqtt::data::MqttPublicInput;
use anyhow::bail;
use log::{debug, error};
use serde_json::Value;
use std::mem::{replace, swap};
use std::path::PathBuf;
use tauri::AppHandle;
use tauri::{command, State};
use tokio::fs;
use tokio::io::AsyncReadExt;
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
    app.brokers
        .iter_mut()
        .find(|x| x.data.id == datas.broker_id)
        .and_then(|x| {
            x.subscribe_topics.push(SubscribeTopic::from(datas.clone()));
            x.data.subscribe_hises.push(datas.clone().into());
            None::<()>
        });
    to_subscribe(&app.mqtt_clients, SubscribeTopic::from(datas)).await;
    Ok(())
}

#[command]
pub async fn publish(datas: PublishInput, state: State<'_, ArcApp>) -> Result<()> {
    debug!("publish: {:?}", datas);
    let mut app = state.write().await;
    // todo history
    // app.brokers
    //     .iter_mut()
    //     .find(|x| x.id == datas.broker_id)
    //     .and_then(|x| {
    //         x.subscribe_topics.push(SubscribeTopic::from(datas.clone()));
    //         x.subscribe_hises.push(datas.clone().into());
    //         None::<()>
    //     });
    to_publish(&app.mqtt_clients, MqttPublicInput::from(datas)).await;
    Ok(())
}

#[command]
pub async fn disconnect(id: usize, state: State<'_, ArcApp>) -> Result<()> {
    debug!("disconnect: {}", id);
    let mut app = state.write().await;
    to_disconnect(&mut app.mqtt_clients, id).await;
    Ok(())
}

#[command]
pub async fn delete_broker(id: usize, state: State<'_, ArcApp>) -> Result<()> {
    debug!("disconnect: {}", id);
    let mut app = state.write().await;
    to_disconnect(&mut app.mqtt_clients, id).await;
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
        //todo to notify frontend
        return Ok(());
    };
    connect(&mut app.mqtt_clients, app_handle, broker).await;
    // app.tx.send(AppEvent::ToConnect(broker)).map_err(|_| {
    //     error!("error");
    //     "send ToConnect event fail".to_string()
    // })?;
    Ok(())
}

#[command]
pub async fn update_or_new_broker(mut broker: BrokerView, state: State<'_, ArcApp>) -> Result<()> {
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
            fs::create_dir_all(broker_path).await;
            // copy to home_path/id
            fs::copy(ca_path, broker_ca_path.as_path()).await?;
        }
        broker.self_signed_ca = broker_ca_path
            .to_str()
            .ok_or(Error::init("self signed ca copied fail"))?
            .to_string();
    }
    let data: BrokerDB = broker.into();
    app.save_broker(data)?;
    Ok(())
}
