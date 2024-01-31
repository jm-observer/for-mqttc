use crate::data::hierarchy::AppData;
use crate::data::{AppEvent, EventUnSubscribe};
use crate::mqtt::{init_connect, mqtt_public, mqtt_subscribe, to_unsubscribe};
// use crate::ui::tabs::init_brokers_tabs;
use crate::data::click_ty::ClickTy;
use crate::data::common::{Broker, QoS, SubscribeHis, SubscribeTopic};
use crate::mqtt::data::MqttPublicInput;
use crate::mqtt::Client;
use crate::ui::ids::{
    SCROLL_MSG_ID, SCROLL_SUBSCRIBE_ID, SELECTOR_AUTO_SCROLL, SELECTOR_TABS_SELECTED, TABS_ID, TIPS,
};

use crate::util::hint::{
    DELETE_BROKER_SUCCESS, DELETE_SUBSCRIBE_SUCCESS, DISCONNECT_SUCCESS, PUBLISH_SUCCESS,
    SAVE_BROKER_SUCCESS, SUBSCRIBE_SUCCESS, UNSUBSCRIBE_SUCCESS,
};

use anyhow::Result;
use bytes::Bytes;
use crossbeam_channel::{Receiver, Sender};
use custom_utils::rx;

use for_mqtt_client::SubscribeAck;
use log::{debug, error, info, warn};
use std::collections::HashMap;

use crate::config::AutoRetract;
use druid::WidgetId;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::Relaxed;
use std::sync::Arc;
use std::time::Duration;
use tokio::spawn;
use tokio::time::sleep;

static CLICK_INFO: AtomicUsize = AtomicUsize::new(0);
static CLICK_LIST: AtomicUsize = AtomicUsize::new(0);

#[tokio::main(flavor = "multi_thread", worker_threads = 10)]
pub async fn deal_event(
    event_sink: druid::ExtEventSink,
    rx: Receiver<AppEvent>,
    tx: Sender<AppEvent>,
    auto_retract: AutoRetract,
) -> Result<()> {
    let mut mqtt_clients: HashMap<usize, Client> = HashMap::new();
    let mut click_his: Option<ClickTy> = None;
    let mut click_broker_info = CLICK_INFO.fetch_add(1, Relaxed);
    let mut click_broker_list = CLICK_LIST.fetch_add(1, Relaxed);

    debug!("{:?}", auto_retract);
    if tx.send(AppEvent::TouchClickBrokerInfo).is_err() {
        error!("fail to send event");
    }
    if tx.send(AppEvent::TouchClickBrokerList).is_err() {
        error!("fail to send event");
    }
    loop {
        // let event = ;
        // debug!("{:?}", event);
        match rx!(rx) {
            // 点击info界面
            AppEvent::TouchClickBrokerInfo => {
                let info_tx = tx.clone();
                click_broker_info = CLICK_INFO.fetch_add(1, Relaxed);
                if let AutoRetract::Open(time) = auto_retract.clone() {
                    spawn(async move {
                        sleep(Duration::from_secs(time)).await;
                        if info_tx
                            .send(AppEvent::TimeoutClickBrokerInfo(click_broker_info))
                            .is_err()
                        {
                            error!("fail to send event");
                        }
                    });
                }
            }
            AppEvent::TimeoutClickBrokerInfo(id) => {
                if click_broker_info == id {
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        if data.broker_tabs.len() > 0 {
                            data.display_broker_info = false;
                        }
                    });
                }
            }
            AppEvent::TouchClickBrokerList => {
                click_broker_list = CLICK_LIST.fetch_add(1, Relaxed);

                let info_tx = tx.clone();
                if let AutoRetract::Open(time) = auto_retract.clone() {
                    spawn(async move {
                        sleep(Duration::from_secs(time)).await;
                        if info_tx
                            .send(AppEvent::TimeoutClickBrokerList(click_broker_list))
                            .is_err()
                        {
                            error!("fail to send event");
                        }
                    });
                }
            }
            AppEvent::TimeoutClickBrokerList(id) => {
                if click_broker_list == id {
                    event_sink.add_idle_callback(move |data: &mut AppData| {
                        if data.broker_tabs.len() > 0 {
                            data.display_history = false;
                        }
                    });
                }
            }
            AppEvent::TouchClickTab(broker_id) => touch_click_tab(&event_sink, broker_id),
            AppEvent::TouchAddBroker => touch_add_broker(&event_sink),
            AppEvent::TouchEditBrokerSelected => edit_broker(&event_sink),
            AppEvent::TouchConnectBrokerSelected => touch_connect_broker_selected(&event_sink),
            AppEvent::TouchSaveBroker => touch_save_broker(&event_sink),
            AppEvent::TouchRemoveSubscribeHis(id) => touch_delete_subscribe_his(&event_sink, id),
            AppEvent::TouchUnSubscribe {
                broker_id,
                trace_id,
            } => touch_unsubscribe(&event_sink, broker_id, trace_id),
            AppEvent::ToUnsubscribeIng(event) => {
                to_unsubscribe_ing(&event_sink, event, &mqtt_clients).await
            }
            AppEvent::ClientUnSubAck(broke_id, unsubscribe_ack) => {
                un_sub_ack(&event_sink, broke_id, unsubscribe_ack.id)
            }
            AppEvent::ToConnect(broker) => {
                connect(&event_sink, &mut mqtt_clients, tx.clone(), broker).await
            }
            AppEvent::TouchConnectByButton => touch_connect_by_button(&event_sink).await,
            AppEvent::TouchSubscribeByInput(index) => {
                touch_subscribe_by_input(&event_sink, index).await
            }
            AppEvent::TouchSubscribeFromHis(his) => {
                touch_subscribe_from_his(&event_sink, his).await
            }
            AppEvent::TouchPublic(broker_id) => {
                if let Err(e) = touch_publish(&event_sink, broker_id).await {
                    error!("{:?}", e);
                }
            }
            AppEvent::ClientReceivePublic(index, topic, payload, qos) => {
                if let Err(e) = receive_public(&event_sink, index, topic, payload, qos).await {
                    error!("{:?}", e);
                }
            }
            AppEvent::ClientPubAck(id, ack) => pub_ack(&event_sink, id, ack),
            AppEvent::ClientSubAck(id, ack) => sub_ack(&event_sink, id, ack),
            AppEvent::UpdateToSelectTabs(id) => update_to_select_tabs(&event_sink, id),
            AppEvent::TouchReConnect => {
                if let Err(e) = touch_reconnect(&event_sink).await {
                    error!("{}", e.to_string());
                }
            }
            AppEvent::TouchDisconnect => {
                if let Err(e) = disconnect(&event_sink).await {
                    error!("{}", e.to_string());
                }
            }
            AppEvent::TouchCloseBrokerTab(id) => {
                touch_close_broker_tab(&event_sink, id);
            }
            // AppEvent::CloseConnectionTab(id) => {
            //     if let Err(e) = close_connection_tab(&event_sink, &mut mqtt_clients, id).await {
            //         error!("{}", e.to_string());
            //     }
            // }
            AppEvent::TouchDeleteBrokerSelected => touch_delete_broker_selected(&event_sink),
            AppEvent::ClientConnectAckSuccess { broker_id, retain } => {
                update_to_connected(&event_sink, broker_id, retain)
            } // _ => {}
            AppEvent::ClientConnectAckFail(_id, _msg) => error!("{}", _msg.to_string()),
            AppEvent::ClientDisconnect(id) => {
                client_disconnect(&event_sink, id);
            }
            AppEvent::ClientConnectedErr(id, msg) => {
                client_connect_err(&event_sink, id, msg);
            }
            AppEvent::UpdateStatusBar(msg) => {
                update_status_bar(&event_sink, msg);
            }
            AppEvent::TouchClearMsg(id) => clear_msg(&event_sink, id),

            AppEvent::UpdateScrollSubscribeWin => scroll_subscribe_win(&event_sink).await,
            AppEvent::UpdateScrollMsgWin => scroll_msg_win(&event_sink).await,
            AppEvent::TouchClick(ty) => {
                if let Some(old_ty) = click_his.take() {
                    if old_ty != ty {
                        click_his = Some(ty)
                    } else {
                        // double click
                        if let Err(e) = double_click(&event_sink, ty).await {
                            error!("{:?}", e);
                        }
                    }
                } else {
                    click_his = Some(ty.clone());
                    first_click(&event_sink, ty.clone()).await;
                    let tx = tx.clone();
                    tokio::spawn(async move {
                        tokio::time::sleep(Duration::from_millis(280)).await;
                        if let Err(e) = tx.send(AppEvent::OtherClickLifeDead(ty)) {
                            error!("{:?}", e);
                        }
                    });
                }
            }
            AppEvent::OtherClickLifeDead(ty) => {
                if let Some(old_ty) = click_his.take() {
                    if old_ty != ty {
                        click_his = Some(old_ty)
                    }
                }
            }
            AppEvent::ToDisconnect(broker_id) => {
                if let Err(e) = to_disconnect(&event_sink, &mut mqtt_clients, broker_id).await {
                    error!("{:?}", e);
                }
            }
            AppEvent::ToSubscribe(input) => {
                to_subscribe(&mqtt_clients, input).await;
            }
            AppEvent::ToPublish(input) => {
                if let Err(e) = to_publish(&mqtt_clients, input).await {
                    error!("{:?}", e);
                }
            }
            AppEvent::OtherDisplayTips => {
                if let Err(e) = event_sink.submit_command(TIPS, (), WidgetId::reserved(0)) {
                    error!("{:?}", e);
                }
            }
        }
    }
}

async fn first_click(event_sink: &druid::ExtEventSink, ty: ClickTy) {
    match ty {
        ClickTy::Broker(id) => {
            click_broker(event_sink, id);
        }
        ClickTy::SubscribeTopic(_, _) => {}
        ClickTy::SubscribeHis(his) => click_subscribe_his(event_sink, his.clone()),
        ClickTy::ConnectTab(broker_id) => touch_click_tab(event_sink, broker_id),
    }
}
async fn double_click(event_sink: &druid::ExtEventSink, ty: ClickTy) -> Result<()> {
    match ty {
        ClickTy::Broker(id) => {
            event_sink.add_idle_callback(move |data: &mut AppData| {
                if let Err(e) = data.db_click_broker(id) {
                    error!("{}", e.to_string());
                }
            });
        }
        ClickTy::SubscribeTopic(broker_id, trace_id) => {
            touch_unsubscribe(&event_sink, broker_id, trace_id);
        }
        ClickTy::SubscribeHis(his) => {
            event_sink.add_idle_callback(move |data: &mut AppData| {
                if let Err(e) = data.touch_subscribe_from_his(his) {
                    error!("{:?}", e);
                }
            });
        }
        ClickTy::ConnectTab(_) => {
            touch_reconnect(event_sink).await?;
        }
    }
    Ok(())
}
async fn scroll_subscribe_win(event_sink: &druid::ExtEventSink) {
    sleep(Duration::from_millis(50)).await;
    if let Err(e) = event_sink.submit_command(SELECTOR_AUTO_SCROLL, (), SCROLL_SUBSCRIBE_ID) {
        error!("{:?}", e);
    }
}
async fn scroll_msg_win(event_sink: &druid::ExtEventSink) {
    sleep(Duration::from_millis(50)).await;
    if let Err(e) = event_sink.submit_command(SELECTOR_AUTO_SCROLL, (), SCROLL_MSG_ID) {
        error!("{:?}", e);
    }
}
fn update_status_bar(event_sink: &druid::ExtEventSink, msg: String) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.hint = msg.into();
    });
}
fn touch_add_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.touch_add_broker();
    });
}
fn edit_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        data.edit_broker();
    });
}

fn touch_connect_broker_selected(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_connect_broker_selected() {
            error!("{:?}", e);
        }
    });
}

fn touch_save_broker(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_save_broker() {
            error!("{:?}", e);
        } else {
            info!("{}", SAVE_BROKER_SUCCESS);
        }
    });
}

fn touch_delete_subscribe_his(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_remove_subscribe_his(id) {
            warn!("{}", e.to_string());
        } else {
            info!("{}", DELETE_SUBSCRIBE_SUCCESS);
        }
    });
}

fn touch_click_tab(event_sink: &druid::ExtEventSink, broker_id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_click_tab(broker_id) {
            warn!("{}", e.to_string());
        }
    });
}

fn touch_unsubscribe(event_sink: &druid::ExtEventSink, broker_id: usize, trace_id: u32) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_unsubscribe(broker_id, trace_id) {
            error!("{:?}", e);
        }
    });
}

async fn to_unsubscribe_ing(
    event_sink: &druid::ExtEventSink,
    event: EventUnSubscribe,
    mqtt_clients: &HashMap<usize, Client>,
) {
    let EventUnSubscribe {
        broke_id,
        subscribe_pk_id,
        topic,
    } = event;
    match to_unsubscribe(broke_id, topic, &mqtt_clients).await {
        Ok(pk_id) => {
            event_sink.add_idle_callback(move |data: &mut AppData| {
                if let Err(e) = data.to_unsubscribe(broke_id, subscribe_pk_id, pk_id) {
                    error!("{:?}", e);
                }
            });
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

fn un_sub_ack(event_sink: &druid::ExtEventSink, broke_id: usize, unsubscribe_pk_id: u32) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.unsubscribe_ack(broke_id, unsubscribe_pk_id) {
            error!("{:?}", e);
        } else {
            info!("{}", UNSUBSCRIBE_SUCCESS)
        }
    });
}

async fn touch_connect_by_button(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.init_connection_for_selected() {
            error!("{:?}", e);
        }
    });
}

async fn connect(
    _event_sink: &druid::ExtEventSink,
    mqtt_clients: &mut HashMap<usize, Client>,
    tx: Sender<AppEvent>,
    broker: Broker,
) {
    if let Some(old_client) = mqtt_clients.remove(&broker.id) {
        if let Err(err) = old_client.disconnect().await {
            error!("diconnect fail: {:?}", err);
        };
    };
    match init_connect(broker.clone(), tx.clone()).await {
        Ok(client) => {
            let id = broker.id;
            mqtt_clients.insert(id, client.clone());
        }
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

async fn touch_subscribe_by_input(event_sink: &druid::ExtEventSink, index: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_subscribe_by_input(index) {
            error!("{:?}", e);
        }
    });
}

async fn to_subscribe(mqtt_clients: &HashMap<usize, Client>, input: SubscribeTopic) {
    match mqtt_subscribe(input.broker_id, input.clone().into(), &mqtt_clients).await {
        Ok(()) => {}
        Err(e) => {
            error!("{:?}", e);
        }
    }
}

async fn touch_subscribe_from_his(event_sink: &druid::ExtEventSink, input: SubscribeHis) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_subscribe_from_his(input) {
            error!("{:?}", e);
        }
    });
}

async fn touch_publish(event_sink: &druid::ExtEventSink, broker_id: usize) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.publish(broker_id) {
            error!("{:?}", e);
        }
    });
    Ok(())
}

async fn to_publish(mqtt_clients: &HashMap<usize, Client>, publish: MqttPublicInput) -> Result<()> {
    mqtt_public(publish.broker_id, publish, &mqtt_clients).await?;
    Ok(())
}

async fn receive_public(
    event_sink: &druid::ExtEventSink,
    index: usize,
    topic: Arc<String>,
    payload: Arc<Bytes>,
    qos: QoS,
) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.receive_msg(index, topic, payload, qos) {
            error!("{:?}", e);
        }
    });
    Ok(())
}

fn pub_ack(event_sink: &druid::ExtEventSink, id: usize, trace_id: u32) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.pub_ack(id, trace_id) {
            error!("{}", e.to_string());
        } else {
            info!("{}", PUBLISH_SUCCESS);
        }
    });
}

fn sub_ack(event_sink: &druid::ExtEventSink, id: usize, ack: SubscribeAck) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.sub_ack(id, ack) {
            error!("{}", e.to_string());
        } else {
            info!("{}", SUBSCRIBE_SUCCESS);
        }
    });
}
fn update_to_select_tabs(event_sink: &druid::ExtEventSink, id: usize) {
    if let Err(e) = event_sink.submit_command(SELECTOR_TABS_SELECTED, id, TABS_ID) {
        error!("{:?}", e);
    }
}

fn click_broker(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.click_broker(id) {
            error!("{:?}", e);
        }
    });
}

fn click_subscribe_his(event_sink: &druid::ExtEventSink, his: SubscribeHis) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.click_subscribe_his(his) {
            error!("{:?}", e);
        }
    });
}

async fn touch_reconnect(event_sink: &druid::ExtEventSink) -> Result<()> {
    // if let Some(client) = mqtt_clients.remove(&id) {
    //     client.disconnect().await?;
    // }
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_reconnect() {
            error!("{}", e.to_string());
        }
    });
    Ok(())
}

async fn to_disconnect(
    _event_sink: &druid::ExtEventSink,
    mqtt_clients: &mut HashMap<usize, Client>,
    id: usize,
) -> Result<()> {
    if let Some(client) = mqtt_clients.remove(&id) {
        client.disconnect().await?;
        info!("{}", DISCONNECT_SUCCESS);
        // 未必有连接，因此无需报警
        // } else {
        //     warn!("could not find mqtt client!");
    }
    Ok(())
}

async fn disconnect(event_sink: &druid::ExtEventSink) -> Result<()> {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_disconnect() {
            error!("{:?}", e);
        }
    });
    Ok(())
}

fn touch_close_broker_tab(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_close_broker_tab(id) {
            error!("{:?}", e);
        }
    });
}

fn touch_delete_broker_selected(event_sink: &druid::ExtEventSink) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.touch_delete_broker_selected() {
            error!("{:?}", e);
        } else {
            info!("{}", DELETE_BROKER_SUCCESS)
        }
    });
}
fn update_to_connected(event_sink: &druid::ExtEventSink, id: usize, retain: bool) {
    info!("connect success!");
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.update_to_connected(id, retain) {
            error!("{:?}", e);
        }
    });
}

fn clear_msg(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.clear_msg(id) {
            error!("{:?}", e);
        } else {
            info!("clear msg success!");
        }
    });
}

fn client_disconnect(event_sink: &druid::ExtEventSink, id: usize) {
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.client_disconnect(id) {
            error!("{:?}", e);
        }
    });
}
fn client_connect_err(event_sink: &druid::ExtEventSink, id: usize, msg: String) {
    error!("{:?}", msg);
    event_sink.add_idle_callback(move |data: &mut AppData| {
        if let Err(e) = data.client_disconnect(id) {
            error!("{:?}", e);
        }
    });
}
