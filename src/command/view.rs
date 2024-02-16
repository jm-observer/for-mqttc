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

#[derive(Serialize, Deserialize)]
struct Root {
    pub name: String,
    pub client_id: String,
    pub addr: String,
    pub port: u16,
    pub auto_connect: bool,
    pub credential: bool,
    pub user_name: String,
    pub password: String,
    pub version: Protocol,
    pub tls: Tls,
    pub self_signed_ca: String,
    pub params: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Tls {
    None,
    Ca,
    Insecurity,
    SelfSigned,
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
            id: *id,
            protocol: *protocol,
            name: name.as_ref(),
            port: *port,
            tls: *tls,
            addr: addr.as_ref(),
        }
    }
}
