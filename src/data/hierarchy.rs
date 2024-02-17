use crate::data::common::Broker;
use crate::data::db::BrokerDB;

use crate::util::db::ArcDb;

use anyhow::bail;
use anyhow::Result;

use for_mqtt_client::Client;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct App {
    pub brokers: Vec<Broker>,
    pub db: ArcDb,
    pub hint: String,
    pub mqtt_clients: HashMap<usize, Client>,
    pub home_path: PathBuf,
}

impl App {
    pub fn save_broker(&mut self, data: BrokerDB) -> Result<()> {
        if self.db.save_broker(&data)? {
            self.brokers.push(data.into_broker());
        } else {
            let Some(broker) = self.brokers.iter_mut().find(|x| x.data.id == data.id) else {
                bail!("could not find broker");
            };
            broker.data = data;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct UnsubcribeTracing {
    pub subscribe_pk_id: u32,
    pub unsubscribe_pk_id: u32,
}
