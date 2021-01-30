use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use super::EnvHandler;
use crate::common::Result;
use crate::handle::insert_source_cmd;
use crate::util::path::to_abs_path;
use async_std::process::Command;
use async_trait::async_trait;
use log::info;
use regex::Regex;
use std::path::Path;

impl Zsh {
    async fn is_installed() -> bool {
        let output = Command::new("cd")
            .arg(to_abs_path("~/.oh-my-zsh"))
            .output()
            .await
            .expect("get output failed");

        output.status.success()
    }

    // zsh的安装，需要将raw.githubxxx.com 添加到host文件中，否则可能会导致curl无法获取到其内容
    async fn pre_install() -> Result<()> {
        // 检查hosts文件是否有raw.githubusercontent.com的配置，若没有，则提示
        let content = async_file::get_content("/etc/hosts").await?;
        let mut has_raw = false;
        for line in content.lines() {
            if line.contains("raw.githubusercontent.com") && !line.starts_with("#") {
                has_raw = true;
            }
        }
        if !has_raw {
            return Err(failure::err_msg(
                "should config raw.githubusercontent.com to /etc/hosts, can use ping to get ip",
            ));
        }

        return Ok(());
    }

    // 安装zsh
    async fn install_zsh() -> Result<()> {
        let output = Command::new("curl")
            .arg("-fsSL")
            .arg("https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh")
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

    // 安装plugin, 返回结果true表示安装成功，false表示已安装过了
    async fn install_plugin(name: &str) -> Result<bool> {
        let zsh_custom = to_abs_path("~/.oh-my-zsh/custom");
        match name {
            "zsh-autosuggestions" | "zsh-syntax-highlighting" => {
                let into_dir = &format!("{}/plugins/{}", zsh_custom, name);
                if Path::new(into_dir).exists() {
                    info!("{} has been installed", name);
                    return Ok(false);
                }
                let output = Command::new("git")
                    .args(&[
                        "clone",
                        &format!("git://github.com/zsh-users/{}", name),
                        into_dir,
                    ])
                    .output()
                    .await?;
                if !output.status.success() {
                    return Err(failure::err_msg(format!("{:?}", output)));
                }
                info!(
                    "install {} success, you should add it into the variable of zshrc plugins",
                    name
                );
            }
            &_ => {}
        };
        Ok(true)
    }

    // 添加插件
    async fn install_plugins(&mut self) -> Result<()> {
        let mut success_names = vec![];
        let empty = &vec![];
        for name in self.plugins.as_ref().unwrap_or(empty) {
            if Self::install_plugin(&name).await? {
                success_names.push(name);
            }
        }

        let mut zshrc_lines = vec![];
        // TODO 安装成功后，将其导入到plugins变量中
        if let Some(zshrc) = self.zshrc.as_ref() {
            let content = async_file::get_content(zshrc).await?;
            let lines = content.split("\n");
            for line in lines {
                if line.starts_with("plugins=(") {
                    zshrc_lines.push(line.to_string());
                    success_names
                        .iter()
                        .for_each(|&name| zshrc_lines.push(String::from(name)));
                    continue;
                }
                zshrc_lines.push(line.to_string());
            }
        }
        let content = zshrc_lines.join("\n");

        Ok(())
    }

    // 处理环境变量
    async fn handle_env(&mut self) -> Result<()> {
        self.zshrc = self.zshrc.take().map(|x| to_abs_path(&x));
        self.zshenv = self.zshenv.take().map(|x| to_abs_path(&x));
        insert_source_cmd(&self.zshrc, "~/.zshrc").await?;
        // 将zshrc文件中的source $ZSH/oh-my-zsh.sh注释掉
        let content = async_file::get_content("~/.zshrc").await?;
        let mut lines = vec![];
        for line in content.lines() {
            if line.trim().eq("source $ZSH/oh-my-zsh.sh") {
                lines.push(format!("#{}", line));
                continue;
            }
            lines.push(line.to_string());
        }
        let done_content = lines.join("\n");
        async_file::save("~/.zshrc", &done_content).await?;
        insert_source_cmd(&self.zshenv, "~/.zshenv").await?;
        Ok(())
    }
}

#[async_trait]
impl EnvHandler for Zsh {
    type Output = ();

    // 安装zsh到self.path对应的目录中去，并将zshenv放入~/.zshenv中去
    async fn handle(&mut self) -> Result<Self::Output> {
        Self::pre_install().await?;
        // 安装zsh: sh -c "$(curl -fsSL https://raw.githubusercontent.com/robbyrussell/oh-my-zsh/master/tools/install.sh)"
        if !Self::is_installed().await {
            Self::install_zsh().await?;
        }
        // 安装成功, 将环境变量导入到~/.zshenv中等
        self.handle_env().await?;
        // 添加插件
        self.install_plugins().await?;
        Ok(())
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[async_std::test]
    async fn test_zsh() {
        let app_conf = AppConf::new("conf.toml");
        let mut zsh = app_conf.zsh.unwrap();
        zsh.handle().await.unwrap();
    }

    #[async_std::test]
    async fn test_zsh_is_install() {
        println!("output = {:?}", Zsh::is_installed().await);
    }

    #[async_std::test]
    async fn test_zsh_install() {
        println!("Zsh::install_zsh().await = {:?}", Zsh::install_zsh().await);
    }

    #[async_std::test]
    async fn test_my_test() {
        println!(
            "std::env::var(\"ZSH_CUSTOM\") = {:?}",
            std::env::var("ZSH_CUSTOM")
        );
    }

    #[async_std::test]
    async fn test_regex() {}
}
