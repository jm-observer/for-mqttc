mod error;
mod view;

use crate::command::error::Error;
use crate::command::view::{BrokerList, BrokerSimpleView, Page};
use crate::data::hierarchy::App;
use crate::data::AppEvent;
use crate::logic::connect;
use log::{debug, error};
use tauri::AppHandle;
use tauri::{command, State};
use tokio::io::AsyncReadExt;
use tokio::sync::RwLock;

type ArcApp = RwLock<App>;
type Result<T> = std::result::Result<T, Error>;

#[command]
pub async fn broker_list(page: Page, state: State<'_, ArcApp>) -> Result<String> {
    let app = state.read().await;
    let total = app.brokers.len();
    let brokers = app.brokers.iter();
    let brokers = brokers.skip(page.start);
    let brokers: Vec<BrokerSimpleView> = brokers
        .take(page.size)
        .map(BrokerSimpleView::from)
        .collect();
    let rs = BrokerList { brokers, total };
    let rs = serde_json::to_string(&rs)?;
    Ok(rs)
}

#[command]
pub async fn connect_to_broker(
    id: usize,
    state: State<'_, ArcApp>,
    app_handle: AppHandle,
) -> Result<()> {
    debug!("connect_to_broker: {id}");
    let mut app = state.write().await;
    let Some(broker) = app
        .brokers
        .iter()
        .find_map(|x| if x.id == id { Some(x.clone()) } else { None })
    else {
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
