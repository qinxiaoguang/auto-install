mod app_conf;
mod common;
mod handle;
mod util;

use crate::handle::EnvHandler;
use app_conf::AppConf;
use async_std::task;
use lazy_static::lazy_static;
use log::info;
use log4rs;

lazy_static! {}

// 初始化日志系统
fn init_logger() {
    log4rs::init_file("log.yaml", Default::default()).unwrap();
    info!("env_logger initialized.");
}

#[async_std::main]
async fn main() {
    init_logger();
    AppConf::new("conf.toml").handle().await;
}
