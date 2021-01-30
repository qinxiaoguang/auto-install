use crate::handle::{EnvHandler, Event};
use crate::join_all;
use async_std::task;
use log::info;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AppConf {
    pub name: Option<String>,
    pub des: Option<String>,
    pub os: Option<String>,
    pub shell_env: Option<ShellEnv>,
    pub zsh: Option<Zsh>,
    pub pkg_manager: Option<PkgManager>,
    pub alacritty: Option<Alacritty>,
    pub tmux: Option<Tmux>,
}

// shell 环境配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct ShellEnv {
    pub bashrc: Option<String>,
    pub bash_profile: Option<String>,
    pub priority: Option<i32>,
}

// zsh 环境配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Zsh {
    pub zshenv: Option<String>,
    pub zshrc: Option<String>,
    pub plugins: Option<Vec<String>>,
    pub priority: Option<i32>,
}

// vim 环境配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Tmux {
    pub tmux_conf: Option<String>,
    pub tmux_conf_local: Option<String>,
    pub priority: Option<i32>,
}

// pkg_manager 环境配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct PkgManager {
    pub use_: Option<String>,
    pub install: Option<Vec<String>>,
    pub cask_install: Option<Vec<String>>,
    pub priority: Option<i32>,
}

// alacritty配置
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Alacritty {
    pub alacritty: Option<String>,
    pub priority: Option<i32>,
}

impl AppConf {
    pub fn new(file_path: &str) -> Self {
        use std::fs::File;
        use std::io::prelude::*;
        let mut file = match File::open(file_path) {
            Ok(f) => f,
            Err(e) => panic!("no such file {} exception:{}", file_path, e),
        };
        let mut str_val = String::new();
        match file.read_to_string(&mut str_val) {
            Ok(s) => s,
            Err(e) => panic!("error reading file: {}", e),
        };
        let app: AppConf = toml::from_str(&str_val).unwrap();
        info!("app_conf is :{:?}", app);
        app
    }

    // 处理
    pub async fn handle(&mut self) {
        let mut itasks = vec![]; // 独立任务，不和其他有优先级的冲突
        let mut ihandlers: Vec<Event> = vec![];
        let mut map: HashMap<i32, Vec<Event>> = HashMap::new();
        self.shell_env.clone().map(|x| {
            if let Some(p) = x.priority {
                map.entry(p).or_insert(vec![]).push(Box::new(x));
            } else {
                ihandlers.push(Box::new(x));
            }
        });
        self.zsh.clone().map(|x| {
            if let Some(p) = x.priority {
                return map.entry(p).or_insert(vec![]).push(Box::new(x));
            }
            ihandlers.push(Box::new(x));
        });
        self.pkg_manager.clone().map(|x| {
            if let Some(p) = x.priority {
                return map.entry(p).or_insert(vec![]).push(Box::new(x));
            }
            ihandlers.push(Box::new(x));
        });
        self.alacritty.clone().map(|x| {
            if let Some(p) = x.priority {
                return map.entry(p).or_insert(vec![]).push(Box::new(x));
            }
            ihandlers.push(Box::new(x));
        });

        self.tmux.clone().map(|x| {
            if let Some(p) = x.priority {
                return map.entry(p).or_insert(vec![]).push(Box::new(x));
            }
            ihandlers.push(Box::new(x));
        });

        for mut handler in ihandlers {
            // 独力任务，直接运行
            // 在task::spawn中内部捕获的变量的生命周期必须是static,所以不能引用外部的变量，除非把外部的变量move进去
            // 而move的前提就是使用clon
            itasks.push(task::spawn(async move { handler.handle().await }));
        }

        // 按照map中的优先级来执行task
        let mut key: Vec<i32> = map.keys().map(|x| *x).collect();
        key.sort();
        for k in key {
            let mut tasks = vec![];
            for mut handler in map.remove(&k).unwrap() {
                tasks.push(task::spawn(async move { handler.handle().await }));
            }
            join_all!(tasks);
        }
        join_all!(itasks);
    }
}

#[cfg(test)]
mod test {
    use super::*;
    #[async_std::test]
    async fn test_app_conf() {
        let mut app_conf = AppConf::new("conf.toml");
        app_conf.handle().await;
        println!("{:?}", app_conf);
    }
}
