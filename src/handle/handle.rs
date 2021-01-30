use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use crate::common::Result;
use async_trait::async_trait;
use log::info;
use std::path::Path;

// 所有的自动装配都要实现handle
#[async_trait]
pub trait EnvHandler {
    type Output;

    // 如果想将该future传给task::spawn的话
    async fn handle(&mut self) -> Result<Self::Output>;
}

pub type Event = Box<dyn EnvHandler<Output = ()> + Send>;
