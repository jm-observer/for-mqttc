use anyhow::Result;
use crossbeam_channel::Sender;
use sled::{Config, Db};
use std::path::PathBuf;
use std::sync::Arc;

use crate::data::common::{Broker, Protocol, PublishInput, SignedTy, SubscribeInput, TabStatus};
use crate::data::db::{BrokerDB, DbKey};
use crate::data::hierarchy::App;
use crate::data::AppEvent;
use log::{debug, warn};

#[derive(Clone, Debug)]
pub struct ArcDb {
    pub index: usize,
    pub db: Db,
    pub ids: Vec<usize>,
}

const BROKERS: &[u8; 7] = b"brokers";
impl ArcDb {
    pub fn init_db(db_path: PathBuf) -> Result<Self> {
        let config = Config::new().path(db_path);
        Ok(ArcDb {
            index: 0,
            db: config.open()?,
            ids: Default::default(),
        })
    }

    pub fn read_app_data(&mut self, tx: Sender<AppEvent>) -> Result<App> {
        // let mut brokers = Vec::new();
        let brokers = if let Some(val) = self.db.get(BROKERS)? {
            let db_brokers_ids: Vec<usize> = serde_json::from_slice(&val)?;
            self.ids = db_brokers_ids.clone();
            debug!("{:?}", self.ids);
            let mut brokers = Vec::new();
            for id in db_brokers_ids.into_iter() {
                if id > self.index {
                    self.index = id;
                }
                if let Some(val) = self.db.get(DbKey::broker_key(id).as_bytes()?)? {
                    let mut broker: BrokerDB = serde_json::from_slice(&val)?;
                    broker.id = id;
                    let mut broker = broker.into_broker(tx.clone());
                    debug!("{:?}", broker);
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
            hint: "".to_string(),
            tx,
            mqtt_clients: Default::default(),
        })
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
    use crate::data::common::{Protocol, SignedTy};
    use crate::data::db::{BrokerDB, Credentials, Tls};
    use crate::util::db::ArcDb;
    use directories::UserDirs;
    use sled::*;
    use std::sync::Arc;

    #[test]
    fn insert_broker() {
        let param = r#"
        {
	"keep_alive": 60,
	"clean_session": true,
	"max_incoming_packet_size": 10240,
	"max_outgoing_packet_size": 10240,
	"inflight": 100,
	"conn_timeout": 5
}
        "#;
        let user_dirs = UserDirs::new().unwrap();
        let home_path = user_dirs.home_dir().to_path_buf().join(".for-mqttc");

        let mut db = ArcDb::init_db(home_path.join("db")).unwrap();
        let broker = BrokerDB {
            id: 1,
            protocol: Protocol::V4,
            client_id: "id_5678".to_string(),
            name: "emq".to_string(),
            addr: "broker-cn.emqx.io".to_string(),
            port: 1883,
            params: param.to_string(),
            credentials: Credentials::None,
            auto_connect: true,
            tls: Tls::None,
            subscribe_hises: vec![],
        };
        db.save_broker(broker).unwrap();
    }
}
