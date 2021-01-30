use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use super::EnvHandler;
use crate::common::Result;
use crate::handle::force_link;
use crate::util::path::to_abs_path;
use async_std::process::Command;
use async_trait::async_trait;
use log::info;
use std::path::Path;

impl Tmux {
    fn link(src_path: &str, dst_path: &str) {
        info!("ready to link {:?} to {:?}", src_path, dst_path);
    }
}
#[async_trait]
impl EnvHandler for Tmux {
    type Output = ();
    async fn handle(&mut self) -> Result<Self::Output> {
        if let Some(res) = self
            .tmux_conf
            .clone()
            .map(|path| force_link(&to_abs_path(&path), &to_abs_path("~/.tmux.conf")))
        {
            if let Err(e) = res {
                return Err(failure::err_msg(format!("err is :{:?}", e)));
            }
        };

        if let Some(res) = self
            .tmux_conf_local
            .clone()
            .map(|path| force_link(&to_abs_path(&path), &to_abs_path("~/.tmux.conf.local")))
        {
            if let Err(e) = res {
                return Err(failure::err_msg(format!("err is :{:?}", e)));
            }
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
        println!(" = {:?}", app_conf.tmux.unwrap().handle().await);
    }
}
