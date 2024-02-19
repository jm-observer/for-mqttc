// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use crate::config::Config;

use crate::util::custom_logger::CustomWriter;
use crate::util::db::ArcDb;
use directories::UserDirs;
use flexi_logger::{Age, Cleanup, Criterion, FileSpec, Naming};

use log::LevelFilter::{Debug, Info};

use tokio::sync::RwLock;

mod data;
mod util;
// mod logic;
mod config;
mod mqtt;

mod command;
mod logic;

use command::*;

fn main() -> anyhow::Result<()> {
    let (tx, _rx) = crossbeam_channel::bounded(1024);

    let user_dirs = UserDirs::new().unwrap();
    let home_path = user_dirs.home_dir().to_path_buf().join(".for-mqttc");

    let fs_path = home_path.clone();
    let fs = FileSpec::default()
        .directory(fs_path)
        .basename("for-mqtt")
        .suffix("log");
    // 若为true，则会覆盖rotate中的数字、keep^
    let criterion = Criterion::AgeOrSize(Age::Day, 10_000_000);
    let naming = Naming::Numbers;
    let cleanup = Cleanup::KeepLogFiles(2);
    let append = true;

    let _logger = custom_utils::logger::logger_feature_with_path(
        "for-mqtt",
        Debug,
        Info,
        home_path.clone(),
        home_path.clone(),
    )
    .module("sled", Info)
    .module("for_event_bus", Info)
    .module("for_mqtt_client::protocol::packet", Info)
    .config(fs, criterion, naming, cleanup, append)
    .log_to_write(Box::new(CustomWriter(tx.clone())))
    .build();

    // panic::set_hook(Box::new(|panic_info| {
    //     error!("{:?}", Backtrace::new());
    //     if let Some(location) = panic_info.location() {
    //         error!(
    //             "panic occurred in file '{}' at line {}",
    //             location.file(),
    //             location.line(),
    //         );
    //     }
    //     exit(1);
    // }));

    let config = Config::init(home_path.clone());
    // if config.display_tips {
    //     config.display_tips = false;
    //     config.clone().update(home_path.clone());
    //     tx.send(AppEvent::OtherDisplayTips).unwrap();
    // }

    // let locale = get_locale();
    // let win = WindowDesc::new(init_layout(tx.clone(), locale.clone())) //.background(B_WINDOW))
    //     .title("for-mqtt")
    //     .window_size((1200.0, 710.0)); //.menu(menu);
    let mut db = ArcDb::init_db(home_path.join("db"))?;
    let data = db.read_app_data(home_path)?;

    let _config_clone = config.clone();

    // thread::Builder::new()
    //     .name("logic-worker".to_string())
    //     .spawn(move || {
    //         if let Err(e) = deal_event(ExtEventSink, rx, tx, config_clone.auto_retract) {
    //             error!("{:?}", e);
    //         }
    //     })
    //     .unwrap();

    tauri::Builder::default()
        .manage(RwLock::new(data))
        .invoke_handler(tauri::generate_handler![
            broker_list,
            connect_to_broker,
            subscribe,
            publish,
            disconnect,
            delete_broker,
            update_or_new_broker,
            loading,
            publish_his,
            subscribe_his,
            unsubscribe
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    Ok(())
}
