use crate::data::common::{PublishInput, QoS, SubscribeTopic};
use bytes::Bytes;
use std::sync::Arc;

#[derive(Debug)]
pub struct MqttPublicInput {
    pub broker_id: usize,
    pub trace_id: u32,
    pub topic: Arc<String>,
    pub msg: Bytes,
    pub qos: QoS,
    pub retain: bool,
}
#[derive(Debug)]
pub struct MqttSubscribeInput {
    pub trace_id: u32,
    pub topic: String,
    pub qos: QoS,
}

impl TryFrom<PublishInput> for MqttPublicInput {
    type Error = anyhow::Error;

    fn try_from(val: PublishInput) -> Result<Self, Self::Error> {
        let msg = val.payload_ty.to_bytes(&val.msg)?;
        Ok(Self {
            broker_id: val.broker_id,
            trace_id: val.trace_id,
            topic: Arc::new(val.topic),
            msg: msg.0,
            qos: val.qos,
            retain: val.retain,
        })
    }
}
impl From<SubscribeTopic> for MqttSubscribeInput {
    fn from(val: SubscribeTopic) -> Self {
        Self {
            trace_id: val.trace_id,
            topic: val.topic.clone(),
            qos: val.qos,
        }
    }
}
// impl From<SubscribeHis> for MqttSubscribeInput {
//     fn from(val: SubscribeHis) -> Self {
//         Self {
//             topic: val.topic.as_ref().clone(),
//             qos: val.qos.into(),
//         }
//     }
// }
