use crate::app_conf::*;
use crate::util::async_file;
//use std::error::Error;
use super::EnvHandler;
use crate::common::Result;
use crate::util::shell::{cp, ln};
use async_trait::async_trait;
use log::info;
use std::path::Path;

// 将source $path 插入到对应的src_file中
pub async fn insert_source_cmd(path: &Option<String>, src_file: &str) -> Result<()> {
    match path {
        Some(rc) => {
            if rc.len() == 0 {
                return Err(failure::err_msg("path cant be empty").into());
            }
            // 将rc对应的目录，放到bashrc文件
            let mut content = async_file::get_content(src_file).await?;
            // 获取content是否有source self.bashrc的内容
            let insert_cmd = format!("source {}", rc);
            info!("insert_cmd \"{}\" to file: \"{}\"", insert_cmd, src_file);
            if !content.contains(&insert_cmd) {
                content = format!("{}\n{}", content, insert_cmd);
                // 将content写入bashrc中
                info!("insert_content {}", content);
                async_file::save(src_file, &content).await?;
                info!("insert success");
            }
            Ok(())
        }
        None => Ok(()),
    }
}

// 强制连接
pub fn force_link(src_path: &str, dst_path: &str) -> Result<()> {
    info!("ready to link {:?} to {:?}", src_path, dst_path);

    // link 前将dst_path备份一便
    if Path::new(&dst_path).exists() {
        cp(&[dst_path, &format!("{}_bak", dst_path)])?;
    }
    // 软连接
    let out = ln(&["-s", "-f", &src_path, &dst_path])?;
    if !out.status.success() {
        return Err(failure::err_msg(String::from_utf8(out.stderr)?));
    }
    info!("link success");
    Ok(())
}
