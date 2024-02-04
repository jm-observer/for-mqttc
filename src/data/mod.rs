pub mod click_ty;
pub mod common;
pub mod db;
pub mod hierarchy;
// pub mod lens;
pub mod localized;

use crate::data::common::{QoS, SubscribeHis, SubscribeStatus, SubscribeTopic};
use bytes::Bytes;
use common::Broker;

use crate::mqtt::data::MqttPublicInput;
use anyhow::bail;
use for_mqtt_client::protocol::packet::SubscribeReasonCode;
use for_mqtt_client::{SubscribeAck, UnsubscribeAck};
use log::{debug, error, warn};
use serde::Serialize;
use serde_json::{Map, Value};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    /// 展示tips
    OtherDisplayTips,
    /// 点击了某个连接tab(broker_id)
    // TouchClickTab(usize),
    /// broker列表的新增图标。新增broker
    TouchAddBroker,
    /// broker列表的编辑图标。编辑选择的broker
    TouchConnectBrokerSelected,
    /// broker列表的删除图标。删除选择的broker
    // TouchDeleteBrokerSelected,
    /// 根据输入进行订阅
    TouchSubscribeByInput(usize),
    TouchSubscribeFromHis(SubscribeHis),
    // e.g: delete broker; close tab; click button "disconnect"
    TouchDisconnect,
    TouchSaveBroker,
    TouchReConnect,
    /// broker信息界面中连接按钮。
    TouchConnectByButton,
    /// 调用第三方库连接broker
    ToConnect(Broker),
    /// 调用第三方库断开连接
    ToDisconnect(usize),
    // select brokers tab
    UpdateToSelectTabs(usize),
    TouchRemoveSubscribeHis(usize),
    /// 通知client进行订阅
    ToSubscribe(SubscribeTopic),
    TouchUnSubscribe {
        broker_id: usize,
        trace_id: u32,
    },
    ToPublish(MqttPublicInput),
    ToUnsubscribeIng(EventUnSubscribe),
    ClientConnectAckSuccess {
        broker_id: usize,
        retain: bool,
    },
    ClientConnectAckFail(usize, String),
    ClientConnectedErr(usize, String),
    ClientDisconnect(usize),
    TouchPublic(usize),
    ClientReceivePublic(usize, Arc<String>, Arc<Bytes>, QoS),
    ClientPubAck(usize, u32),
    ClientSubAck {
        broker_id: usize,
        ack: SubscribeAck,
    },
    ClientUnSubAck(usize, UnsubscribeAck),
    // TouchClick(ClickTy),
    // OtherClickLifeDead(ClickTy),
    TouchCloseBrokerTab(usize),
    // CloseConnectionTab(usize),
    UpdateStatusBar(String),
    /// 清空消息
    TouchClearMsg(usize),
    /// 滚动消息窗口
    UpdateScrollMsgWin,
    /// 滚动订阅窗口
    UpdateScrollSubscribeWin,
}

#[derive(Default)]
struct EventBuilder {
    map: Map<String, Value>,
}
impl EventBuilder {
    pub fn with_param(mut self, name: &str, val: impl Into<Value>) -> Self {
        self.map.insert(name.to_string(), val.into());
        self
    }
    pub fn build(self) -> Value {
        Value::Object(self.map)
    }
}

impl AppEvent {
    pub fn event(self) -> Option<(&'static str, Option<Value>)> {
        use AppEvent::*;
        debug!("build event: {:?}", self);
        Some(match self {
            ClientConnectAckSuccess { broker_id, retain } => {
                let event = EventBuilder::default()
                    .with_param("broker_id", broker_id)
                    .with_param("retain", retain)
                    .build();
                ("ClientConnectAckSuccess", Some(event))
            }
            ClientConnectAckFail(id, msg) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", id)
                    .with_param("msg", msg)
                    .build();
                ("ClientConnectAckFail", Some(event))
            }
            ClientPubAck(id, packet_id) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", id)
                    .with_param("packet_id", packet_id as usize)
                    .build();
                ("ClientPubAck", Some(event))
            }
            ClientSubAck { broker_id, mut ack } => {
                let Some(reason) = ack.acks.pop() else {
                    error!("get subscribe reason fail");
                    return None;
                };
                let event = get_subscribe_rs(
                    reason,
                    EventBuilder::default()
                        .with_param("broker_id", broker_id)
                        .with_param("trace_id", ack.id as usize),
                )
                .build();
                ("ClientSubAck", Some(event))
            }
            ClientUnSubAck(_id, _ack) => {
                todo!()
            }
            ClientReceivePublic(_id, _ack, ..) => {
                todo!()
            }
            ClientConnectedErr(id, msg) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", id)
                    .with_param("msg", msg)
                    .build();
                ("ClientConnectedErr", Some(event))
            }
            ClientDisconnect(broker_id) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", broker_id)
                    .build();
                ("ClientDisconnect", Some(event))
            }
            _ => {
                todo!()
            }
        })
    }
}
#[derive(Debug, Clone)]
pub struct EventUnSubscribe {
    pub broke_id: usize,
    pub subscribe_pk_id: u32,
    pub topic: String,
}

fn get_subscribe_rs(ack: SubscribeReasonCode, builder: EventBuilder) -> EventBuilder {
    match ack {
        SubscribeReasonCode::QoS0 => builder.with_param("success", true).with_param("qos", 0),
        SubscribeReasonCode::QoS1 => builder.with_param("success", true).with_param("qos", 1),
        SubscribeReasonCode::QoS2 => builder.with_param("success", true).with_param("qos", 2),
        _ => builder
            .with_param("success", false)
            .with_param("msg", format!("{:?}", ack)),
    }
}
