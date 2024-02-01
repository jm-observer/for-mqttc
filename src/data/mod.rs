pub mod click_ty;
pub mod common;
pub mod db;
pub mod hierarchy;
// pub mod lens;
pub mod localized;

use crate::data::click_ty::ClickTy;
use crate::data::common::{QoS, SubscribeHis, SubscribeTopic};
use bytes::Bytes;
use common::Broker;

use crate::mqtt::data::MqttPublicInput;
use for_mqtt_client::{SubscribeAck, UnsubscribeAck};
use std::sync::Arc;

pub type AString = Arc<String>;

#[derive(Debug)]
pub enum AppEvent {
    /// 展示tips
    OtherDisplayTips,
    /// 点击了某个连接tab(broker_id)
    TouchClickTab(usize),
    /// broker列表的新增图标。新增broker
    TouchAddBroker,
    /// broker列表的编辑图标。编辑选择的broker
    TouchEditBrokerSelected,
    /// broker列表的连接图标。连接选择的broker
    TouchConnectBrokerSelected,
    /// broker列表的删除图标。删除选择的broker
    TouchDeleteBrokerSelected,
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
    ClientConnectAckFail(usize, Arc<String>),
    ClientConnectedErr(usize, String),
    ClientDisconnect(usize),
    TouchPublic(usize),
    ClientReceivePublic(usize, Arc<String>, Arc<Bytes>, QoS),
    ClientPubAck(usize, u32),
    ClientSubAck(usize, SubscribeAck),
    ClientUnSubAck(usize, UnsubscribeAck),
    TouchClick(ClickTy),
    OtherClickLifeDead(ClickTy),
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
#[derive(Debug, Clone)]
pub struct EventUnSubscribe {
    pub broke_id: usize,
    pub subscribe_pk_id: u32,
    pub topic: String,
}
