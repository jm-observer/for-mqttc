use crate::data::common::{
    Broker, Protocol, PublishInput, SignedTy, SubscribeHis, SubscribeInput, TabStatus,
};
use crate::data::AppEvent;
use anyhow::Result;
use crossbeam_channel::Sender;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DbKey {
    Broker(usize),
    SubscribeHis(usize),
}

impl DbKey {
    pub fn broker_key(id: usize) -> Self {
        Self::Broker(id)
    }
    pub fn subscribe_his_key(id: usize) -> Self {
        Self::SubscribeHis(id)
    }
    pub fn as_bytes(&self) -> Result<Vec<u8>> {
        Ok(serde_json::to_vec(self)?)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerDB {
    pub id: usize,
    pub protocol: Protocol,
    pub client_id: String,
    pub name: String,
    pub addr: String,
    pub port: u16,
    pub params: String,
    pub credentials: Credentials,
    pub auto_connect: bool,
    pub tls: Tls,
    #[serde(default)]
    pub subscribe_hises: Vec<SubscribeHis>,
}

impl BrokerDB {
    pub fn into_broker(self, tx: Sender<AppEvent>) -> Broker {
        Broker {
            data: self,
            tx,
            subscribe_topics: Default::default(),
            msgs: Default::default(),
            unsubscribe_ing: Default::default(),
        }
    }
}
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Credentials {
    None,
    Credentials { user_name: String, password: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Tls {
    None,
    Ca,
    Insecurity,
    SelfSigned { self_signed_ca: String },
}

impl ToString for Tls {
    fn to_string(&self) -> String {
        match self {
            Tls::None => "none".to_string(),
            Tls::Ca => "ca".to_string(),
            Tls::Insecurity => "insecurity".to_string(),
            Tls::SelfSigned { .. } => "self signed".to_string(),
        }
    }
}
