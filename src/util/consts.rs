use crate::data::common::QoS;
use for_mqtt_client::QoSWithPacketId;
use lazy_static::lazy_static;
use std::sync::Arc;
lazy_static! {
    pub static ref QOS_0: Arc<String> = Arc::new("0".to_string());
}
lazy_static! {
    pub static ref QOS_1: Arc<String> = Arc::new("1".to_string());
}
lazy_static! {
    pub static ref QOS_2: Arc<String> = Arc::new("2".to_string());
}
lazy_static! {
    pub static ref TY_TEXT: Arc<String> = Arc::new("T".to_string());
}
lazy_static! {
    pub static ref TY_JSON: Arc<String> = Arc::new("J".to_string());
}
lazy_static! {
    pub static ref TY_HEX: Arc<String> = Arc::new("H".to_string());
}

pub trait QosToString {
    fn qos_to_string(&self) -> String;
}
impl QosToString for QoS {
    fn qos_to_string(&self) -> String {
        match self {
            QoS::AtMostOnce => "0".to_string(),
            QoS::AtLeastOnce => "1".to_string(),
            QoS::ExactlyOnce => "2".to_string(),
        }
    }
}
impl QosToString for for_mqtt_client::QoS {
    fn qos_to_string(&self) -> String {
        match self {
            for_mqtt_client::QoS::AtMostOnce => "0".to_string(),
            for_mqtt_client::QoS::AtLeastOnce => "1".to_string(),
            for_mqtt_client::QoS::ExactlyOnce => "2".to_string(),
        }
    }
}
impl QosToString for QoSWithPacketId {
    fn qos_to_string(&self) -> String {
        match self {
            QoSWithPacketId::AtMostOnce => "0".to_string(),
            QoSWithPacketId::AtLeastOnce(_) => "1".to_string(),
            QoSWithPacketId::ExactlyOnce(_) => "2".to_string(),
        }
    }
}
