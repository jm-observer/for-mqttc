use crate::data::common::SubscribeHis;

#[derive(Debug, Clone, Eq, PartialEq)]
pub enum ClickTy {
    Broker(usize),
    SubscribeTopic(usize, u32),
    SubscribeHis(SubscribeHis),
    ConnectTab(usize),
}
