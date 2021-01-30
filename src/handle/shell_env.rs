use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use super::common::insert_source_cmd;
use super::EnvHandler;
use crate::common::Result;
use crate::util::path::to_abs_path;
use async_trait::async_trait;
use log::info;
use std::path::Path;

#[async_trait]
impl EnvHandler for ShellEnv {
    type Output = ();
    async fn handle(&mut self) -> Result<Self::Output> {
        // shell的env环境，需要在bash_profile中执行 source来更新
        //Self::handle_file(&self.bashrc, "~/.bashrc")?;
        insert_source_cmd(&self.bashrc.as_ref().map(|x| to_abs_path(x)), "~/.bashrc").await?;
        insert_source_cmd(
            &self.bash_profile.as_ref().map(|x| to_abs_path(x)),
            "~/.bash_profile",
        )
        .await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_shell_env() {
        let app_conf = AppConf::new("conf.toml");
        let mut shell_env = app_conf.shell_env.unwrap();
        shell_env.handle().await.unwrap();
    }
}
