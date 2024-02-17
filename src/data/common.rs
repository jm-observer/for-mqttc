mod impls;

use crate::data::db::BrokerDB;
use crate::data::hierarchy::UnsubcribeTracing;
use crate::data::AppEvent;
use crate::util::consts::{TY_HEX, TY_JSON, TY_TEXT};
use anyhow::bail;
use bytes::Bytes;
use crossbeam_channel::Sender;
use log::{debug, error};
use pretty_hex::simple_hex;
use serde::{Deserialize, Serialize};
use serde_json::Value;

use crate::util::general_id;
use serde_repr::{Deserialize_repr, Serialize_repr};
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
    pub topic: String,
    /// 只针对通配符的topic
    // pub sub_topic: String,
    pub qos: QoS,
    pub status: SubscribeStatus,
    pub payload_ty: PayloadTy,
}
#[derive(Debug, Clone, Eq, PartialEq, Deserialize, Serialize)]
pub struct SubscribeHis {
    pub(crate) topic: String,
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
    pub topic: String,
    pub msg: String,
    pub qos: String,
    pub status: PublicStatus,
    pub payload_ty: String,
    pub time: String,
}
#[derive(Debug, Clone, Eq, PartialEq)]
pub enum PublicStatus {
    Ing,
    Success,
}

#[derive(Debug, Clone, Deserialize)]
pub struct PublishInput {
    pub broker_id: usize,
    pub topic: String,
    pub msg: String,
    pub qos: QoS,
    pub retain: bool,
    pub trace_id: u32,
    pub payload_ty: PayloadTy,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SubscribeMsg {
    pub topic: String,
    pub msg: String,
    pub qos: String,
    pub payload_ty: String,
    pub time: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct SubscribeInput {
    pub broker_id: usize,
    pub trace_id: u32,
    pub topic: String,
    pub qos: QoS,
    pub payload_ty: PayloadTy,
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
#[derive(Debug, Clone, Eq, PartialEq, Deserialize_repr, Serialize_repr, Default)]
#[allow(clippy::enum_variant_names)]
#[repr(u8)]
pub enum QoS {
    #[default]
    AtMostOnce = 0,
    AtLeastOnce = 1,
    ExactlyOnce = 2,
}

#[derive(Debug, Clone)]
pub struct Broker {
    pub data: BrokerDB,
    pub tx: Sender<AppEvent>,
    pub subscribe_topics: Vec<SubscribeTopic>,
    pub msgs: Vec<Msg>,
    pub unsubscribe_ing: Vec<UnsubcribeTracing>,
}

impl Broker {
    pub fn init_connection(&mut self) -> anyhow::Result<()> {
        if self.data.client_id.is_empty() {
            self.data.client_id = general_id();
        }

        if self.data.addr.is_empty() {
            bail!("addr not be empty");
        }
        Ok(())
    }
    pub fn clone_to_db(&self) -> BrokerDB {
        self.data.clone()
    }

    pub fn disconnect(&mut self, clear: bool) {
        if !self.data.auto_connect {
            self.subscribe_topics.clear();
        }
        if clear {
            self.msgs.clear();
        }
        self.unsubscribe_ing.clear();
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
    pub fn to_arc_string(&self) -> String {
        match self {
            PayloadTy::Text => "T".to_string(),
            PayloadTy::Json => "J".to_string(),
            PayloadTy::Hex => "H".to_string(),
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
#[serde(rename_all = "lowercase")]
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
