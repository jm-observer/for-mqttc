use anyhow::Result;
use crossbeam_channel::Sender;
use sled::{Config, Db};
use std::path::PathBuf;
use std::sync::Arc;

use crate::data::common::{Broker, Protocol, PublicInput, SignedTy, SubscribeInput, TabStatus};
use crate::data::db::{BrokerDB, DbKey};
use crate::data::hierarchy::App;
use crate::data::AppEvent;
use log::{debug, warn};

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub index: usize,
    pub db: Db,
    pub tx: Sender<AppEvent>,
    pub ids: Vec<usize>,
}

const BROKERS: &[u8; 7] = b"brokers";
impl ArcDb {
    pub fn init_db(tx: Sender<AppEvent>, db_path: PathBuf) -> Result<Self> {
        let config = Config::new().path(db_path);
        Ok(ArcDb {
            index: 0,
            db: config.open()?,
            tx,
            ids: Default::default(),
        })
    }

    pub fn read_app_data(&mut self) -> Result<App> {
        // let mut brokers = Vec::new();
        let brokers = if let Some(val) = self.db.get(BROKERS)? {
            let db_brokers_ids: Vec<usize> = serde_json::from_slice(&val)?;
            self.ids = db_brokers_ids.clone();
            debug!("{:?}", self.ids);
            let mut brokers = Vec::new();
            let mut index = 0;
            for id in db_brokers_ids.into_iter() {
                index += 1;
                if id > self.index {
                    self.index = id;
                }
                if let Some(val) = self.db.get(DbKey::broker_key(id).as_bytes()?)? {
                    let broker: BrokerDB = serde_json::from_slice(&val)?;
                    let mut broker = broker.into_broker(self.tx.clone());
                    debug!("{:?}", broker);
                    if index == 1 {
                        broker.selected = true;
                    }
                    brokers.push(broker);
                } else {
                    warn!("can't find id: {}", id);
                };
            }
            brokers
        } else {
            Vec::new()
        };
        Ok(App {
            brokers,
            db: self.clone(),
            hint: "".to_string().into(),
            tx: self.tx.clone(),
        })
    }

    pub fn new_broker(&mut self) -> Broker {
        self.index += 1;
        let id = self.index;
        Broker {
            id,
            protocol: Protocol::V4,
            client_id: Arc::new("".to_string()),
            name: Arc::new("".to_string()),
            addr: Arc::new("broker-cn.emqx.io".to_string()),
            port: Some(1883),
            params: Arc::new(OPTION.to_string()),
            use_credentials: false,
            user_name: Arc::new("".to_string()),
            password: Arc::new("".to_string()),
            stored: false,
            tx: self.tx.clone(),
            selected: false,
            tls: false,
            signed_ty: SignedTy::Ca,
            self_signed_ca: Arc::new("".to_string()),
            subscribe_hises: Default::default(),
            subscribe_topics: Default::default(),
            msgs: Default::default(),
            subscribe_input: SubscribeInput::init(id),
            public_input: PublicInput::default(id),
            unsubscribe_ing: Default::default(),
            tab_status: TabStatus {
                id,
                try_connect: false,
                connected: false,
            },
            auto_connect: true,
        }
    }

    pub fn save_broker(&mut self, broker: BrokerDB) -> Result<()> {
        debug!("save broker: {:?}", broker);
        let id = broker.id;
        if !self.ids.iter().any(|x| *x == id) {
            self.ids.push(id);
            self.db.insert(BROKERS, serde_json::to_vec(&self.ids)?)?;
        }
        self.db.insert(
            DbKey::broker_key(id).as_bytes()?,
            serde_json::to_vec(&broker)?,
        )?;
        Ok(())
    }
    pub fn delete_broker(&mut self, id: usize) -> Result<()> {
        let mut selected_index = None;
        for (index, broker) in self.ids.iter().enumerate() {
            if *broker == id {
                selected_index = Some(index);
                break;
            }
        }
        if let Some(index) = selected_index {
            self.ids.remove(index);
            self.update_ids()?;
            self.db.remove(DbKey::broker_key(id).as_bytes()?)?;
        } else {
            warn!("not selected broker to delete");
        }
        Ok(())
    }
    #[inline]
    fn update_ids(&self) -> Result<()> {
        self.db.insert(BROKERS, serde_json::to_vec(&self.ids)?)?;
        Ok(())
    }
    // pub fn update_subscribe_his(&self, id: usize, hises: &Vec<SubscribeHis>) -> Result<()> {
    //     let key = DbKey::subscribe_his_key(id);
    //     self.db
    //         .insert(key.as_bytes()?, serde_json::to_vec(hises)?)?;
    //     Ok(())
    // }
}

const OPTION: &str = r#"{
	"keep_alive": 60,
	"clean_session": true,
	"max_incoming_packet_size": 10240,
	"max_outgoing_packet_size": 10240,
	"inflight": 100,
	"conn_timeout": 5
}
        "#;

#[cfg(test)]
mod test {

    #[test]
    fn insert_broker() {
        // let db = Config::new().path("./resource/db").open().unwrap();
        // let broker = vector![Broker {
        //     id: 0,
        //     client_id: Arc::new("id_5678".to_string()),
        //     name: Arc::new("emq".to_string()),
        //     addr: Arc::new("192.168.199.188".to_string()),
        //     port: Arc::new("1883".to_string()),
        //     params: Arc::new("{abc,jiofewki, iowoere}".to_string()),
        // }];
        // let broker = serde_json::to_vec(&broker).unwrap();
        // db.insert(BROKERS, broker).unwrap();
    }
}
