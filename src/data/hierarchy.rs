use crate::data::common::{Broker, Id, PayloadTy, QoS};
use crate::data::common::{
    Msg, PublicMsg, PublicStatus, SubscribeHis, SubscribeMsg, SubscribeStatus, SubscribeTopic,
};
use crate::data::{AString, AppEvent, EventUnSubscribe};
use crate::mqtt::data::MqttPublicInput;
use crate::util::consts::QosToString;
use crate::util::db::ArcDb;
use crate::util::hint::*;
use crate::util::now_time;
use anyhow::Result;
use anyhow::{anyhow, bail};
use bytes::Bytes;
use crossbeam_channel::Sender;
use custom_utils::tx;
use for_mqtt_client::protocol::packet::SubscribeReasonCode;
use for_mqtt_client::SubscribeAck;
use log::{debug, error, warn};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct App {
    pub brokers: Vec<Broker>,
    pub db: ArcDb,
    pub hint: AString,
    pub tx: Sender<AppEvent>,
}

impl App {
    pub(crate) fn client_disconnect(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        broker.disconnect(false);
        Ok(())
    }
    fn send_event(&self, event: AppEvent) {
        if let Err(e) = self.tx.send(event) {
            error!("fail to send event: {:?}", e.0)
        }
    }
    pub fn touch_add_broker(&mut self) {
        // self.unselect_broker();
        // if let Some(broker) = self.brokers.iter_mut().find(|x| x.stored == false) {
        //     debug!("had a new broker");
        //     broker.selected = true;
        //     self.display_broker_info = true;
        // } else {
        //     debug!("new_broker");
        //     let mut broker = self.db.new_broker();
        //     broker.selected = true;
        //     self.brokers.push_front(broker);
        //     self.display_broker_info = true;
        // }
    }
    /// 取消所有选中
    fn init_broker_tab(&mut self, id: usize) -> bool {
        // let mut is_exist = false;
        // if self.broker_tabs.iter().find(|x| **x == id).is_none() {
        //     self.broker_tabs.push_front(id);
        // } else {
        //     is_exist = true;
        // }
        // is_exist
        todo!()
    }
    pub fn get_selected_subscribe_his(&self) -> Result<SubscribeHis> {
        // let broker = self.get_selected_broker()?;
        // if let Some(his) = broker.subscribe_hises.iter().find(|x| x.selected) {
        //     return Ok(his.clone());
        // }
        bail!("could not find  subscribe his selected");
    }

    pub fn find_broker_by_id(&self, id: usize) -> Result<&Broker> {
        self.brokers
            .iter()
            .find(|x| x.id == id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn find_mut_broker_by_id(&mut self, id: usize) -> Result<&mut Broker> {
        self.brokers
            .iter_mut()
            .find(|x| x.id == id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn find_broker_by_index(&self, id: usize) -> Result<&Broker> {
        self.brokers
            .get(id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn find_mut_broker_by_index(&mut self, id: usize) -> Result<&mut Broker> {
        self.brokers
            .get_mut(id)
            .ok_or(anyhow!("could not find broker:{}", id))
    }
    pub fn touch_save_broker(&mut self) -> Result<()> {
        todo!()
        // let broker = self.get_selected_mut_broker()?;
        // broker.stored = true;
        // let broker = broker.clone_to_db();
        // self.db.save_broker(broker)?;
        // Ok(())
    }
    pub fn touch_reconnect(&mut self) -> Result<()> {
        todo!()
        // let broker = self.get_selected_mut_broker()?;
        // broker.disconnect(false);
        // broker.init_connection()?;
        // let broker = broker.clone();
        // self.disconnect(broker.id)?;
        // self.init_connection_by_broker(broker)?;
        // Ok(())
    }
    pub fn init_connection_for_selected(&mut self) -> Result<()> {
        todo!()
        // let broker = self.get_selected_mut_broker()?;
        // broker.init_connection()?;
        // let broker = broker.clone();
        // self.init_connection_by_broker(broker)?;
        // Ok(())
    }

    fn init_connection_by_broker(&mut self, broker: Broker) -> Result<()> {
        let broker_db = broker.clone_to_db();
        let broker = broker.clone();
        self.init_broker_tab(broker.id);
        self.db.save_broker(broker_db)?;
        self.send_event(AppEvent::ToConnect(broker));
        Ok(())
    }

    pub fn update_to_connected(&mut self, id: usize, _retain: bool) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        let status = &mut broker.tab_status;
        status.try_connect = false;
        status.connected = true;
        if !_retain {
            broker.subscribe_topics.clear();
        }
        Ok(())
    }
    pub(crate) fn touch_disconnect(&mut self) -> Result<()> {
        todo!()
        // let broker = self.get_selected_mut_broker()?;
        // broker.disconnect(false);
        // let id = broker.id;
        // self.disconnect(id)
    }
    fn disconnect(&self, id: usize) -> Result<()> {
        self.send_event(AppEvent::ToDisconnect(id));
        Ok(())
    }
    pub fn close_connection(&mut self, id: usize) -> Result<()> {
        let status = &mut self.find_mut_broker_by_id(id)?.tab_status;
        status.try_connect = false;
        status.connected = false;
        Ok(())
    }
    pub fn unsubscribe(
        &mut self,
        broker_id: usize,
        subscribe_pkid: u32,
        unsubscribe_pkid: u32,
    ) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(broker_id)?;
        _broker.unsubscribe_ing.push(UnsubcribeTracing {
            subscribe_pk_id: subscribe_pkid,
            unsubscribe_pk_id: unsubscribe_pkid,
        });
        Ok(())
    }

    pub fn unsubscribe_ack(&mut self, broker_id: usize, unsubscribe_trace_id: u32) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(broker_id)?;
        if let Some(index) = _broker
            .unsubscribe_ing
            .iter()
            .enumerate()
            .find(|(_index, x)| x.unsubscribe_pk_id == unsubscribe_trace_id)
            .map(|(index, _x)| index)
        {
            let tracing = _broker.unsubscribe_ing.remove(index);
            if let Some(index) = _broker
                .subscribe_topics
                .iter_mut()
                .enumerate()
                .find(|(_index, his)| his.trace_id == tracing.subscribe_pk_id)
                .map(|(index, _x)| index)
            {
                _broker.subscribe_topics.remove(index);
                Ok(self.db.tx.send(AppEvent::UpdateScrollSubscribeWin)?)
            } else {
                bail!("can't find broker's subscribe");
            }
        } else {
            bail!("can't find broker's unsubscribe_tracing");
        }
    }
    pub fn touch_unsubscribe(&mut self, broker_id: usize, trace_id: u32) -> Result<()> {
        let _broker = self.find_mut_broker_by_id(broker_id)?;
        if let Some(index) = _broker
            .subscribe_topics
            .iter_mut()
            .find(|his| his.trace_id == trace_id)
        {
            index.status = SubscribeStatus::UnSubscribeIng;
            let event = EventUnSubscribe {
                broke_id: broker_id,
                subscribe_pk_id: index.trace_id,
                topic: index.topic.as_ref().clone(),
            };
            self.send_event(AppEvent::ToUnsubscribeIng(event));
            return Ok(());
        }
        warn!("can't find the subscribe to unsubscibe");
        Ok(())
    }

    fn subscribe(&mut self, sub: SubscribeTopic) -> Result<()> {
        let id = sub.broker_id;
        let broker = self.find_mut_broker_by_id(id)?;
        if !broker.subscribe_topics.iter().any(|x| x.is_equal(&sub)) {
            broker.subscribe_topics.push(sub.clone());
        } else if let Some((index, _)) = broker
            .subscribe_topics
            .iter()
            .enumerate()
            .find(|(_index, x)| x.topic == sub.topic)
        {
            broker.subscribe_topics.remove(index);
            broker.subscribe_topics.push(sub.clone());
        }

        let his: SubscribeHis = sub.clone().into();
        if !broker.subscribe_hises.iter().any(|x| *x == his) {
            broker.subscribe_hises.push(his);
            let broker = broker.clone_to_db();
            self.db.save_broker(broker)?;
        }
        self.db.tx.send(AppEvent::ToSubscribe(sub))?;
        self.db.tx.send(AppEvent::UpdateScrollSubscribeWin)?;

        Ok(())
    }
    pub fn touch_subscribe_from_his(&mut self, input: SubscribeHis) -> Result<()> {
        debug!("{:?}", input);
        self.subscribe(SubscribeTopic::from_his(input, Id::to_id()))?;
        Ok(())
    }

    pub fn touch_subscribe_by_input(&mut self, id: usize) -> Result<()> {
        let input = self.find_broker_by_id(id)?.subscribe_input.clone();
        self.subscribe(SubscribeTopic::from(input, Id::to_id()))?;
        Ok(())
    }

    // pub fn subscribe_by_input(
    //     &mut self,
    //     id: usize,
    //     input: SubscribeInput,
    //     trace_id: u32,
    // ) -> Result<()> {
    //     self.subscribe(id, SubscribeTopic::from(input.clone(), trace_id))?;
    //     let broker = self.find_mut_broker_by_id(id)?;
    //     let his: SubscribeHis = input.into();
    //     if broker.subscribe_hises.iter().find(|x| *x == &his).is_none() {
    //         broker.subscribe_hises.push(his.into());
    //     }
    //     Ok(self.db.tx.send(AppEvent::ScrollSubscribeWin)?)
    // }

    pub fn touch_remove_subscribe_his(&mut self, id: usize) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        if let Some(index) = broker
            .subscribe_hises
            .iter()
            .enumerate()
            .find(|(_index, his)| his.selected)
            .map(|(index, _his)| index)
        {
            broker.subscribe_hises.remove(index);
            let broker = broker.clone_to_db();
            self.db.save_broker(broker)?;
            return Ok(());
        }
        warn!("{}", DELETE_SUBSCRIBE_NO_SELECTED);
        Ok(())
    }

    pub fn sub_ack(&mut self, id: usize, input: SubscribeAck) -> Result<()> {
        let broker = self.find_mut_broker_by_id(id)?;
        let SubscribeAck { id, mut acks } = input;
        if let Some(ack) = acks.pop() {
            if let Some(subscribe_topic) = broker
                .subscribe_topics
                .iter_mut()
                .find(|x| x.trace_id == id)
            {
                match ack {
                    SubscribeReasonCode::QoS0 => {
                        subscribe_topic.qos = QoS::AtMostOnce.clone();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::QoS1 => {
                        subscribe_topic.qos = QoS::AtLeastOnce.clone();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    SubscribeReasonCode::QoS2 => {
                        subscribe_topic.qos = QoS::ExactlyOnce.clone();
                        subscribe_topic.status = SubscribeStatus::SubscribeSuccess;
                    }
                    _reasone => {
                        subscribe_topic.status = SubscribeStatus::SubscribeFail;
                    }
                }
            } else {
                warn!("could not find subscribe");
            }
        }
        Ok(())
        // } else {
        //     warn!("could not find subscribe");
        // }
    }
    pub fn publish(&mut self, id: usize) -> Result<()> {
        todo!()
        // let broker = self.find_mut_broker_by_id(id)?;
        // let (payload, payload_str) = broker
        //     .public_input
        //     .payload_ty
        //     .to_bytes(&broker.public_input.msg)?;
        // let trace_id = Id::to_id();
        // let msg = PublicMsg {
        //     trace_id,
        //     topic: broker.public_input.topic.clone(),
        //     msg: Arc::new(payload_str),
        //     qos: broker.public_input.qos.qos_to_string(),
        //     status: PublicStatus::Ing,
        //     payload_ty: broker.public_input.payload_ty.to_arc_string(),
        //     time: Arc::new(now_time()),
        // };
        // debug!("publish: tarce_id {}", trace_id);
        //
        // broker.msgs.push(msg.into());
        // if broker.msgs.len() > 50 {
        //     broker.msgs.pop_front();
        // }
        //
        // let publish = MqttPublicInput {
        //     broker_id: broker.id,
        //     trace_id,
        //     topic: broker.public_input.topic.clone(),
        //     msg: payload,
        //     qos: broker.public_input.qos.clone(),
        //     retain: broker.public_input.retain,
        // };
        //
        // self.send_event(AppEvent::ToPublish(publish));
        // self.send_event(AppEvent::UpdateScrollMsgWin);
        // Ok(())
    }
    pub fn touch_connect_broker_selected(&mut self) -> Result<()> {
        self.init_connection_for_selected()?;
        Ok(())
    }
    pub fn db_click_broker(&mut self, id: usize) -> Result<()> {
        // 若已存在，则跳转至该tag；重连。否则，新增tag，连接
        let broker = self.find_mut_broker_by_id(id)?;
        broker.disconnect(false);
        broker.init_connection()?;
        let broker = broker.clone();
        self.disconnect(broker.id)?;
        self.init_connection_by_broker(broker)?;
        // if self.init_broker_tab(id) {
        //     self.db.tx.send(AppEvent::ReConnect(id))?;
        // } else {
        //     let broker = self.find_broker_by_id(id)?;
        //     self.db.tx.send(AppEvent::ToDisconnect(id))?;
        // }
        Ok(())
    }

    pub fn touch_close_broker_tab(&mut self, id: usize) -> Result<()> {
        self.find_mut_broker_by_id(id)?.disconnect(true);
        self.disconnect(id)?;
        Ok(())
    }

    pub fn pub_ack(&mut self, id: usize, trace_id: u32) -> Result<()> {
        debug!("pub_ack: tarce_id {}", trace_id);
        let broker = self.find_mut_broker_by_id(id)?;
        let mut is_ack = false;
        for msg in broker.msgs.iter_mut() {
            if let Msg::Public(msg) = msg {
                if msg.trace_id == trace_id {
                    is_ack = true;
                    msg.status = PublicStatus::Success;
                }
            }
        }
        if !is_ack {
            bail!("pub_ack could not find pub({})", trace_id);
        }
        Ok(())
    }
    pub fn receive_msg(
        &mut self,
        id: usize,
        topic: Arc<String>,
        payload: Arc<Bytes>,
        qos: QoS,
    ) -> Result<()> {
        todo!()
        // let broker = self.find_mut_broker_by_id(id)?;
        // let payload_ty = if let Some(subscribe) = broker
        //     .subscribe_topics
        //     .iter()
        //     .find(|x| x.match_topic(topic.as_str()))
        // {
        //     subscribe.payload_ty.clone()
        // } else {
        //     warn!("could not find this publish's subscribe record");
        //     PayloadTy::default()
        // };
        // let payload = payload_ty.format(payload);
        // let msg = SubscribeMsg {
        //     topic,
        //     msg: Arc::new(payload),
        //     qos: qos.qos_to_string(),
        //     payload_ty: payload_ty.to_arc_string(),
        //     time: Arc::new(now_time()),
        // };
        // broker.msgs.push(msg.into());
        // if broker.msgs.len() > 50 {
        //     broker.msgs.pop_front();
        // }
        // Ok(self.db.tx.send(AppEvent::UpdateScrollMsgWin)?)
    }
    pub fn clear_msg(&mut self, id: usize) -> Result<()> {
        self.find_mut_broker_by_id(id)?.msgs.clear();
        Ok(self.db.tx.send(AppEvent::UpdateScrollMsgWin)?)
    }

    // pub fn msgs_ref_mut(&mut self, id: usize) -> &mut Vec<Msg> {
    //     if !self.msgs.contains_key(&id) {
    //         self.msgs.insert(id, Vec::new());
    //     }
    //     let Some(msgs) = self.msgs.get_mut(&id) else {
    //             unreachable!()
    //     };
    //     msgs
    // }
    // pub fn msgs_ref(&self, id: usize) -> &Vec<Msg> {
    //     if let Some(msgs) = self.msgs.get(&id) {
    //         msgs
    //     } else {
    //         unreachable!()
    //     }
    // }
}
#[derive(Debug, Clone)]
pub struct UnsubcribeTracing {
    pub subscribe_pk_id: u32,
    pub unsubscribe_pk_id: u32,
}
