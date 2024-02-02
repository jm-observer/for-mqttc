mod error;
mod view;

use crate::command::error::Error;
use crate::command::view::{BrokerList, BrokerSimpleView, Page};
use crate::data::hierarchy::App;
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
