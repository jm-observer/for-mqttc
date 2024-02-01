mod impls;

use crate::data::db::BrokerDB;
use crate::data::hierarchy::UnsubcribeTracing;
use crate::data::{AString, AppEvent};
use crate::util::consts::{TY_HEX, TY_JSON, TY_TEXT};
use anyhow::bail;
use bytes::Bytes;
use crossbeam_channel::Sender;
use log::{debug, error};
use pretty_hex::simple_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::general_id;
use std::sync::atomic::{AtomicU32, Ordering};
use std::sync::Arc;

static U32: AtomicU32 = AtomicU32::new(0);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Id(u32);

impl Id {
    pub fn to_id() -> u32 {
        Self::default().0
    }
}

impl Default for Id {
    fn default() -> Self {
        Self(U32.fetch_add(1, Ordering::Release))
    }
}

#[derive(Clone, Debug)]
pub struct SubscribeTopic {
    pub broker_id: usize,
    pub trace_id: u32,

    pub topic: AString,

    /// 只针对通配符的topic
    // pub sub_topic: AString,
    pub qos: QoS,
    pub status: SubscribeStatus,
    pub payload_ty: PayloadTy,
}
#[derive(Debug, Clone, Eq, Deserialize, Serialize)]
pub struct SubscribeHis {
    // #[serde(skip)]
    // pub(crate) id: Id,
    pub(crate) broker_id: usize,
    #[serde(skip)]
    pub(crate) selected: bool,
    pub(crate) topic: AString,
    pub(crate) qos: QoS,
    pub payload_ty: PayloadTy,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum Msg {
    Public(PublicMsg),
    Subscribe(SubscribeMsg),
}

impl Msg {
    pub fn is_public(&self) -> bool {
        if let Msg::Public(_) = self {
            return true;
        }
        false
    }
    pub fn is_sucess(&self) -> bool {
        if let Msg::Public(msg) = self {
            msg.status == PublicStatus::Success
        } else {
            true
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PublicMsg {
    pub trace_id: u32,
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
    pub status: PublicStatus,
    pub payload_ty: AString,
    pub time: AString,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PublicStatus {
    Ing,
    Success,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct PublicInput {
    pub broker_id: usize,
    pub topic: AString,
    pub msg: AString,
    pub qos: QoS,
    pub retain: bool,
    pub payload_ty: PayloadTy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub topic: AString,
    pub msg: AString,
    pub qos: AString,
    pub payload_ty: AString,
    pub time: AString,
}

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct SubscribeInput {
    pub(crate) broker_id: usize,
    pub(crate) topic: AString,
    pub(crate) qos: QoS,
    pub(crate) payload_ty: PayloadTy,
}

impl SubscribeInput {
    pub fn init(broker_id: usize) -> Self {
        Self {
            broker_id,
            topic: Arc::new("".to_string()),
            qos: Default::default(),
            payload_ty: Default::default(),
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum SubscribeStatus {
    SubscribeIng,
    SubscribeSuccess,
    SubscribeFail,
    UnSubscribeIng,
}

#[derive(Debug, Clone)]
pub struct TabStatus {
    pub(crate) id: usize,
    pub(crate) try_connect: bool,
    pub(crate) connected: bool,
}

#[derive(Clone, Copy, Eq, PartialEq, Debug, Hash)]
pub enum TabKind {
    Connection,
    Broker,
}
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize, Default)]
#[repr(u8)]
pub enum QoS {
    #[default]
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Debug, Clone)]
pub struct Broker {
    pub id: usize,
    pub protocol: Protocol,
    pub client_id: AString,
    pub name: AString,
    pub addr: AString,
    pub port: Option<u16>,
    pub params: AString,
    pub use_credentials: bool,
    pub auto_connect: bool,
    pub user_name: AString,
    pub password: AString,
    pub stored: bool,
    pub tx: Sender<AppEvent>,
    pub selected: bool,
    pub tls: bool,
    pub signed_ty: SignedTy,
    pub self_signed_ca: AString,

    pub subscribe_hises: Vec<SubscribeHis>,
    pub subscribe_topics: Vec<SubscribeTopic>,
    pub msgs: Vec<Msg>,
    pub subscribe_input: SubscribeInput,
    pub public_input: PublicInput,
    pub unsubscribe_ing: Vec<UnsubcribeTracing>,
    pub tab_status: TabStatus,
}

impl Broker {
    pub fn init_connection(&mut self) -> anyhow::Result<()> {
        if self.client_id.as_str().is_empty() {
            self.client_id = general_id().into();
        }

        if self.addr.is_empty() {
            bail!("addr not be empty");
        } else if self.port.is_none() {
            bail!("port not be empty");
        } else if self.use_credentials {
            if self.user_name.is_empty() {
                bail!("user name not be empty");
            } else if self.password.is_empty() {
                bail!("password not be empty");
            }
        } else if self.tls
            && self.signed_ty == SignedTy::SelfSigned
            && self.self_signed_ca.is_empty()
        {
            bail!("self signed ca not be empty");
        }
        self.tab_status.try_connect = true;
        self.stored = true;
        Ok(())
    }
    pub fn clone_to_db(&self) -> BrokerDB {
        BrokerDB {
            id: self.id,
            protocol: self.protocol,
            client_id: self.client_id.clone(),
            name: self.name.clone(),
            addr: self.addr.clone(),
            port: self.port,
            params: self.params.clone(),
            use_credentials: self.use_credentials,
            user_name: self.user_name.clone(),
            password: self.password.clone(),
            tls: self.tls,
            signed_ty: self.signed_ty,
            self_signed_ca: self.self_signed_ca.clone(),
            subscribe_hises: self.subscribe_hises.clone(),
            auto_connect: self.auto_connect,
        }
    }

    pub fn disconnect(&mut self, clear: bool) {
        self.tab_status.try_connect = false;
        self.tab_status.connected = false;
        if !self.auto_connect {
            self.subscribe_topics.clear();
        }
        if clear {
            self.msgs.clear();
        }
        self.unsubscribe_ing.clear();
    }
}

impl PartialEq for SubscribeHis {
    fn eq(&self, other: &Self) -> bool {
        self.broker_id == other.broker_id
            && self.topic == other.topic
            && self.qos == other.qos
            && self.payload_ty == other.payload_ty
    }
}
impl ToString for QoS {
    fn to_string(&self) -> String {
        match self {
            QoS::AtMostOnce => "0".to_string(),
            QoS::AtLeastOnce => "1".to_string(),
            QoS::ExactlyOnce => "2".to_string(),
        }
    }
}
impl QoS {
    pub fn to_u8(&self) -> u8 {
        match self {
            QoS::AtMostOnce => 0,
            QoS::AtLeastOnce => 1,
            QoS::ExactlyOnce => 2,
        }
    }
}
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
/// 消息的格式：普通字符串、json字符串、hex
pub enum PayloadTy {
    Text,
    Json,
    Hex,
}

impl PayloadTy {
    pub fn to_arc_string(&self) -> Arc<String> {
        match self {
            PayloadTy::Text => TY_TEXT.clone(),
            PayloadTy::Json => TY_JSON.clone(),
            PayloadTy::Hex => TY_HEX.clone(),
        }
    }
    pub fn format(&self, data: Arc<Bytes>) -> String {
        match self {
            PayloadTy::Text => String::from_utf8_lossy(data.as_ref()).to_string(),
            PayloadTy::Json => match String::from_utf8(data.to_vec()) {
                Ok(rs) => {
                    let Ok(json) = serde_json::from_str::<Value>(rs.as_str()) else {
                        return rs;
                    };
                    let Ok(json) = serde_json::to_string_pretty(&json) else {
                        return rs;
                    };
                    json
                }
                Err(err) => {
                    error!("{}", err.to_string());
                    let rs = String::from_utf8_lossy(data.as_ref()).to_string();
                    rs
                }
            },
            PayloadTy::Hex => simple_hex(data.as_ref()),
        }
    }
    pub fn to_bytes(&self, msg: &String) -> anyhow::Result<(Bytes, String)> {
        Ok(match self {
            PayloadTy::Text => (Bytes::from(msg.as_bytes().to_vec()), msg.clone()),
            PayloadTy::Json => (
                Bytes::from(msg.as_bytes().to_vec()),
                to_pretty_json_from_str(msg.as_str())?,
            ),
            PayloadTy::Hex => {
                let mut chars = msg.chars();
                let mut hex_datas = Vec::with_capacity(chars.clone().count());
                let mut data_str = String::with_capacity(msg.len());
                // 去除非16进制字符，且暂时将16进制转成8进制
                let mut len = 0;
                for c in chars {
                    if c.is_ascii_hexdigit() {
                        let Some(digit) = c.to_digit(16) else {
                            debug!("{} to_digit fail", c);
                            continue;
                        };
                        hex_datas.push(digit as u8);
                        len += 1;
                        data_str.push(c);
                        if len % 2 == 0 {
                            data_str.push(' ');
                        }
                    }
                }
                // 判定长度
                if len % 2 != 0 {
                    bail!("ascii_hexdigit len % 2 != 0!");
                }
                // 合并2位16进制至8进制
                let mut i = 0;
                let mut datas = Vec::with_capacity(len / 2);
                while i < len {
                    // debug!("{} {} {}", hex_datas[i], hex_datas[i] << 4, hex_datas[i + 1], )
                    datas.push((hex_datas[i] << 4) | (hex_datas[i + 1]));
                    i += 2;
                }
                (datas.into(), data_str)
            }
        })
    }
}

// fn to_pretty_json(data: &Arc<Bytes>) -> anyhow::Result<String> {
//     let json = serde_json::from_slice::<Value>(data.as_ref())?;
//     serde_json::Ok(serde_json::to_string_pretty(&json)?)
// }

fn to_pretty_json_from_str(data: &str) -> anyhow::Result<String> {
    let json = serde_json::from_str::<Value>(data)?;
    Ok(serde_json::to_string_pretty(&json)?)
}

impl Default for PayloadTy {
    fn default() -> Self {
        Self::Text
    }
}

/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum Protocol {
    V4,
    V5,
}

/// Protocol type
#[derive(Debug, Clone, Copy, PartialEq, Eq, Deserialize, Serialize)]
pub enum SignedTy {
    Ca,
    SelfSigned,
    Insecurity,
}
