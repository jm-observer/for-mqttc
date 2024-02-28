use crate::data::common::{Broker, PublishHis, PublishInput, SubscribeHis, SubscribeInput};
use crate::data::db::BrokerDB;

use crate::util::db::ArcDb;

use anyhow::anyhow;
use anyhow::Result;

use for_mqtt_client::Client;
use std::collections::HashMap;
use std::mem::swap;
use std::path::PathBuf;

pub struct App {
    pub brokers: Vec<Broker>,
    pub db: ArcDb,
    pub mqtt_clients: HashMap<usize, Client>,
    pub home_path: PathBuf,
    pub hint: String,
}
impl App {
    pub fn save_broker(&mut self, mut data: BrokerDB) -> Result<()> {
        if let Some(broker) = self.brokers.iter_mut().find(|x| x.data.id == data.id) {
            swap(&mut broker.data.subscribe_his, &mut data.subscribe_his);
            swap(&mut broker.data.publish_his, &mut data.publish_his);
            self.db.save_broker(&data)?;
            broker.data = data;
        } else {
            self.db.save_broker(&data)?;
            self.brokers.push(data.into_broker(self.db.clone()));
        }
        Ok(())
    }

    fn find_mut_broker_by_id(&mut self, id: usize) -> Result<&mut Broker> {
        self.brokers
            .iter_mut()
            .find(|x| x.data.id == id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn update_subscribe_his(&mut self, sub: SubscribeInput) -> Result<()> {
        let id = sub.broker_id;
        let broker = self.find_mut_broker_by_id(id)?;
        let his: SubscribeHis = sub.into();
        if !broker.data.subscribe_his.iter().any(|x| *x == his) {
            broker.data.subscribe_his.push(his);
            while broker.data.subscribe_his.len() > 15 {
                broker.data.subscribe_his.remove(0);
            }
            broker.db.save_broker(&broker.data)?;
        }
        Ok(())
    }
    pub fn update_publish_his(&mut self, sub: PublishInput) -> Result<()> {
        let id = sub.broker_id;
        let broker = self.find_mut_broker_by_id(id)?;
        let his: PublishHis = sub.into();
        if !broker.data.publish_his.iter().any(|x| *x == his) {
            broker.data.publish_his.push(his);
            while broker.data.publish_his.len() > 15 {
                broker.data.publish_his.remove(0);
            }
            broker.db.save_broker(&broker.data)?;
        }
        Ok(())
    }
}
#[derive(Debug, Clone)]
pub struct UnsubcribeTracing {
    pub subscribe_pk_id: u32,
    pub unsubscribe_pk_id: u32,
}
