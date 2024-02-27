use crate::command::view::{BrokerView, ClientTlsView, TlsView};
use crate::data::common::{Broker, Protocol, PublishHis, SubscribeHis};

use anyhow::Result;

use crate::util::db::ArcDb;
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
    pub client_tls: ClientTls,
    #[serde(default)]
    pub subscribe_his: Vec<SubscribeHis>,
    #[serde(default)]
    pub publish_his: Vec<PublishHis>,
}

impl BrokerDB {
    pub fn into_broker(self, db: ArcDb) -> Broker {
        Broker { data: self, db }
    }
}

impl From<BrokerView> for BrokerDB {
    fn from(value: BrokerView) -> Self {
        let BrokerView {
            id,
            name,
            client_id,
            addr,
            port,
            auto_connect,
            credential,
            user_name,
            password,
            version,
            tls,
            self_signed_ca,
            params,
            certificate,
            private_key,
            client_tls,
        } = value;
        let credentials = if credential {
            Credentials::Credentials {
                user_name,
                password,
            }
        } else {
            Credentials::None
        };
        let tls = match tls {
            TlsView::None => Tls::None,
            TlsView::Ca => Tls::Ca,
            TlsView::Insecurity => Tls::Insecurity,
            TlsView::SelfSigned => Tls::SelfSigned { self_signed_ca },
        };
        let client_tls = match client_tls {
            ClientTlsView::None => ClientTls::None,
            ClientTlsView::Verify => ClientTls::Verify {
                certificate,
                private_key,
            },
        };
        Self {
            id,
            protocol: version,
            client_id,
            name,
            addr,
            port,
            params,
            credentials,
            auto_connect,
            tls,
            subscribe_his: vec![],
            publish_his: vec![],
            client_tls,
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

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientTls {
    None,
    Verify {
        certificate: String,
        private_key: String,
    },
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
