use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use super::common::insert_source_cmd;
use super::EnvHandler;
use crate::common::Result;
use crate::util::path::to_abs_path;
use async_std::process::Command;
use async_trait::async_trait;
use log::info;
use std::path::Path;

impl PkgManager {
    async fn is_brew_installed() -> Result<bool> {
        let out = Command::new("brew").arg("--version").output().await?;
        Ok(out.status.success())
    }

    // 安装brew
    async fn brew_install() -> Result<()> {
        // 参考: https://zhuanlan.zhihu.com/p/90508170
        let output = Command::new("curl")
            .arg("-fsSL")
            .arg("https://cdn.jsdelivr.net/gh/ineo6/homebrew-install/install.sh")
            .output()
            .await
            .expect("get failed");
        if !output.status.success() {
            return Err(failure::err_msg(format!("{:?}", output)));
        }
        let output = Command::new("sh")
            .arg("-c")
            .arg(String::from_utf8(output.stdout).expect("get stdout failed"))
            .output()
            .await
            .expect("install failed");
        if !output.status.success() {
            return Err(failure::err_msg(format!("{:?}", output)));
        }

        Ok(())
    }

    // 安装brew插件
    async fn install_plugins(&mut self) -> Result<()> {
        if let Some(data) = self.install.as_ref() {
            for plugin in data {
                // 安装plugin
                Self::install_plugin(plugin, false).await?;
            }
        }

        if let Some(data) = self.cask_install.as_ref() {
            for plugin in data {
                // 安装plugin
                Self::install_plugin(plugin, true).await?;
            }
        }

        Ok(())
    }

    async fn install_plugin(name: &str, use_cask: bool) -> Result<()> {
        if Self::plugin_installed(name, use_cask).await? {
            info!("plugin {:?} has been installed", name);
            return Ok(());
        }
        let mut cmd_mid = Command::new("brew");
        let cmd = if use_cask {
            cmd_mid.arg("cask").arg("install")
        } else {
            cmd_mid.arg("install")
        };
        let output = cmd.arg(name).output().await?;
        info!(
            "installing plugin: {:?}, install cmd:brew install {:?}",
            name, name
        );
        if !output.status.success() {
            return Err(failure::err_msg(format!("{:?}", output)));
        }
        info!("plugin {:?} install success", name);
        return Ok(());
    }

    // 判断插件是否已经安装
    async fn plugin_installed(name: &str, use_cask: bool) -> Result<bool> {
        let output = Command::new("brew")
            .arg("list")
            .arg(if use_cask { "--cask" } else { "--formula" })
            .output()
            .await?;
        if !output.status.success() {
            return Err(failure::err_msg(format!("{:?}", output)));
        }

        let list_content = String::from_utf8(output.stdout)?;
        match name {
            _ if name.starts_with("python3") && list_content.contains("python@3") => Ok(true),
            _ if list_content.contains(name) => Ok(true),
            _ => Ok(false),
        }
    }
}
#[async_trait]
impl EnvHandler for PkgManager {
    type Output = ();
    async fn handle(&mut self) -> Result<Self::Output> {
        if let Some(use_name) = self.use_.as_ref() {
            return match use_name.as_str() {
                // 目前只支持brew
                "brew" => {
                    // 判断brew 是否安装
                    if !Self::is_brew_installed().await? {
                        // 未安装，先进行安装
                        Self::brew_install().await?;
                    }

                    // 安装成功，或已存在, 开始安装插件
                    self.install_plugins().await?;
                    Ok(())
                }
                _ => Ok(()),
            };
        }
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_brew_install() {
        let out = Command::new("brew")
            .arg("--version")
            .output()
            .await
            .unwrap();
        println!("out = {:?}", out);
    }

    #[async_std::test]
    async fn test_plugin_installed() {
        println!("{:?}", PkgManager::plugin_installed("python3", false).await);
        println!(
            "{:?}",
            PkgManager::plugin_installed("autojump", false).await
        );
        println!("{:?}", PkgManager::plugin_installed("haha", false).await);
        println!(
            "{:?}",
            PkgManager::plugin_installed("alacritty", true).await
        );
    }

    #[async_std::test]
    async fn test_plugin_install() {
        println!(
            "{:?}",
            PkgManager::install_plugin("autojump", false).await.unwrap()
        );
    }
}
