use crate::config::Config;
use crate::data::common::Protocol;
use crate::data::db::{BrokerDB, ClientTls, Credentials, Tls};
use crate::data::hierarchy::App;
use serde::{Deserialize, Serialize};
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ViewConfig {
    hint: String,
    debug: bool,
}

impl ViewConfig {
    pub fn init(app: &App, config: &Config) -> Self {
        Self {
            hint: app.hint.clone(),
            debug: config.debug,
        }
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct BrokerList {
    pub brokers: Vec<BrokerView>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BrokerView {
    pub id: usize,
    pub name: String,
    pub client_id: String,
    pub addr: String,
    pub port: u16,
    pub auto_connect: bool,
    pub credential: bool,
    pub user_name: String,
    pub password: String,
    pub version: Protocol,
    pub tls: TlsView,
    pub self_signed_ca: String,
    pub params: String,
    pub certificate: String,
    pub private_key: String,
    pub client_tls: ClientTlsView,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ClientTlsView {
    None,
    Verify,
}

impl ClientTlsView {
    pub fn is_verify(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TlsView {
    None,
    Ca,
    Insecurity,
    SelfSigned,
}

impl TlsView {
    pub fn is_tls(&self) -> bool {
        match self {
            Self::None => false,
            _ => true,
        }
    }
}

impl From<BrokerDB> for BrokerView {
    fn from(value: BrokerDB) -> Self {
        let BrokerDB {
            id,
            protocol,
            client_id,
            name,
            addr,
            port,
            params,
            credentials,
            auto_connect,
            tls,
            client_tls,
            ..
        } = value;
        let (credential, user_name, password) = match credentials {
            Credentials::None => (false, "".to_string(), "".to_string()),
            Credentials::Credentials {
                user_name,
                password,
            } => (true, user_name, password),
        };
        let (tls, self_signed_ca) = match tls {
            Tls::None => (TlsView::None, "".to_string()),
            Tls::Ca => (TlsView::Ca, "".to_string()),
            Tls::Insecurity => (TlsView::Insecurity, "".to_string()),
            Tls::SelfSigned { self_signed_ca } => (TlsView::SelfSigned, self_signed_ca),
        };
        let (certificate, private_key, client_tls) = match client_tls {
            ClientTls::None => ("".to_string(), "".to_string(), ClientTlsView::None),
            ClientTls::Verify {
                certificate,
                private_key,
            } => (certificate, private_key, ClientTlsView::Verify),
        };
        Self {
            id,
            version: protocol,
            name,
            port,
            auto_connect,
            credential,
            user_name,
            password,
            tls,
            self_signed_ca,
            addr,
            client_id,
            params,
            certificate,
            private_key,
            client_tls,
        }
    }
}
