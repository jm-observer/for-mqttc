use crate::data::common::{Broker, Protocol};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Page {
    pub start: usize,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrokerList<'a> {
    pub brokers: Vec<BrokerSimpleView<'a>>,
    pub total: usize,
}

#[derive(Debug, Clone, Serialize)]
pub struct BrokerSimpleView<'a> {
    pub id: usize,
    pub protocol: Protocol,
    pub name: &'a String,
    pub addr: &'a String,
    pub port: Option<u16>,
    pub tls: bool,
}

impl<'a> From<&'a Broker> for BrokerSimpleView<'a> {
    fn from(value: &'a Broker) -> Self {
        let Broker {
            id,
            protocol,
            name,
            addr,
            port,
            tls,
            ..
        } = value;
        Self {
            id: 0,
            protocol: *protocol,
            name: name.as_ref(),
            port: *port,
            tls: *tls,
            addr: addr.as_ref(),
        }
    }
}
