use crate::data::common::{
    Msg, PublicMsg, QoS, SubscribeHis, SubscribeInput, SubscribeMsg, SubscribeTopic,
};
use crate::mqtt;

impl SubscribeTopic {
    pub fn from(val: SubscribeInput) -> Self {
        Self {
            broker_id: val.broker_id,
            trace_id: val.trace_id,
            topic: val.topic.clone(),
            qos: val.qos.clone(),
            payload_ty: val.payload_ty,
        }
    }
}

impl From<SubscribeInput> for SubscribeHis {
    fn from(val: SubscribeInput) -> Self {
        Self {
            topic: val.topic.clone(),
            qos: val.qos.clone(),
            payload_ty: val.payload_ty,
        }
    }
}

impl From<SubscribeTopic> for SubscribeHis {
    fn from(val: SubscribeTopic) -> Self {
        Self {
            topic: val.topic.clone(),
            qos: val.qos.clone(),
            payload_ty: val.payload_ty,
        }
    }
}

impl From<PublicMsg> for Msg {
    fn from(val: PublicMsg) -> Self {
        Self::Public(val)
    }
}
impl From<SubscribeMsg> for Msg {
    fn from(val: SubscribeMsg) -> Self {
        Self::Subscribe(val)
    }
}

// impl Default for SubscribeInput {
//     fn default() -> Self {
//         Self {
//             topic: Arc::new("".to_string()),
//             qos: QoS::AtMostOnce,
//             payload_ty: Default::default(),
//         }
//     }
// }

impl From<mqtt::QoS> for QoS {
    fn from(qos: mqtt::QoS) -> Self {
        match qos {
            mqtt::QoS::AtLeastOnce => Self::AtLeastOnce,
            mqtt::QoS::AtMostOnce => Self::AtMostOnce,
            mqtt::QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
impl From<mqtt::QoSWithPacketId> for QoS {
    fn from(qos: mqtt::QoSWithPacketId) -> Self {
        match qos {
            mqtt::QoSWithPacketId::AtLeastOnce(_) => Self::AtLeastOnce,
            mqtt::QoSWithPacketId::AtMostOnce => Self::AtMostOnce,
            mqtt::QoSWithPacketId::ExactlyOnce(_) => Self::ExactlyOnce,
        }
    }
}
impl From<QoS> for mqtt::QoS {
    fn from(qos: QoS) -> Self {
        match qos {
            QoS::AtLeastOnce => Self::AtLeastOnce,
            QoS::AtMostOnce => Self::AtMostOnce,
            QoS::ExactlyOnce => Self::ExactlyOnce,
        }
    }
}
