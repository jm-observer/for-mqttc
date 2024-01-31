use crate::data::common::{QoS, SubscribeTopic};
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

// impl From<PublicInput> for MqttPublicInput {
//     fn from(val: PublicInput) -> Self {
//         Self {
//             topic: val.topic.as_ref().clone(),
//             msg: val.msg.as_ref().clone(),
//             qos: QoS::AtLeastOnce,
//             retain: val.retain,
//             payload_ty: val.payload_ty,
//         }
//     }
// }
impl From<SubscribeTopic> for MqttSubscribeInput {
    fn from(val: SubscribeTopic) -> Self {
        Self {
            trace_id: val.trace_id,
            topic: val.topic.as_ref().clone(),
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
