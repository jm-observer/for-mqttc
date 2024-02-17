pub mod common;
pub mod db;
pub mod hierarchy;
// pub mod lens;
pub mod localized;

use crate::data::common::QoS;
use bytes::Bytes;

use for_mqtt_client::protocol::packet::SubscribeReasonCode;
use for_mqtt_client::{SubscribeAck, UnsubscribeAck};
use log::{debug, error};

use serde_json::{Map, Number, Value};
use std::sync::Arc;

#[derive(Debug)]
pub enum AppEvent {
    ConnectAckSuccess {
        broker_id: usize,
        retain: bool,
    },
    ConnectAckFail(usize, String),
    ConnectedErr(usize, String),
    Disconnect(usize),
    ReceivePublic {
        broker_id: usize,
        topic: Arc<String>,
        payload: Arc<Bytes>,
        qos: QoS,
    },
    PubAck(usize, u32),
    SubAck {
        broker_id: usize,
        ack: SubscribeAck,
    },
    UnSubAck(usize, UnsubscribeAck),
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
            ConnectAckSuccess { broker_id, retain } => {
                let event = EventBuilder::default()
                    .with_param("broker_id", broker_id)
                    .with_param("retain", retain)
                    .build();
                ("ClientConnectAckSuccess", Some(event))
            }
            ConnectAckFail(id, msg) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", id)
                    .with_param("msg", msg)
                    .build();
                ("ClientConnectAckFail", Some(event))
            }
            PubAck(id, packet_id) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", id)
                    .with_param("trace_id", packet_id as usize)
                    .build();
                ("ClientPubAck", Some(event))
            }
            SubAck { broker_id, mut ack } => {
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
            UnSubAck(_id, _ack) => {
                todo!()
            }
            ReceivePublic {
                broker_id,
                topic,
                payload,
                qos,
            } => {
                let payload = payload
                    .iter()
                    .copied()
                    .map(|x| Value::Number(Number::from(x)))
                    .collect::<Vec<Value>>();
                let event = EventBuilder::default()
                    .with_param("broker_id", broker_id)
                    .with_param("topic", topic.as_ref().clone())
                    .with_param("payload", payload)
                    .with_param("qos", qos as usize)
                    .build();
                ("ClientReceivePublic", Some(event))
            }
            ConnectedErr(id, msg) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", id)
                    .with_param("msg", msg)
                    .build();
                ("ClientConnectedErr", Some(event))
            }
            Disconnect(broker_id) => {
                let event = EventBuilder::default()
                    .with_param("broker_id", broker_id)
                    .build();
                ("ClientDisconnect", Some(event))
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
