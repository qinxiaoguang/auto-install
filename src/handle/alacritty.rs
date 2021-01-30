use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use super::force_link;
use super::EnvHandler;
use crate::common::Result;
use crate::util::path::to_abs_path;
use async_std::process::Command;
use async_trait::async_trait;
use failure::Error;
use log::info;
use std::path::Path;

#[async_trait]
impl EnvHandler for Alacritty {
    type Output = ();
    async fn handle(&mut self) -> Result<Self::Output> {
        match self.alacritty.clone().map(|path| {
            force_link(
                &to_abs_path(&path),
                &to_abs_path("~/.config/alacritty/alacritty.yml"),
            )
        }) {
            Some(res) => match res {
                Err(e) => return Err(failure::err_msg(format!("err is :{:?}", e))),
                _ => {}
            },
            _ => {}
        };
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_alacritty() {
        let mut app_conf = AppConf::new("conf.toml");
        println!(" = {:?}", app_conf.alacritty.unwrap().handle().await);
    }
}
