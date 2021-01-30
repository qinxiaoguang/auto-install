//util 和common的区别就是util不依赖该项目模块，而common与该项目有互相的依赖
pub mod alacritty;
pub mod common;
pub mod handle;
pub mod pkg_manager;
pub mod shell_env;
pub mod tmux;
pub mod zsh;

pub use alacritty::*;
pub use common::*;
pub use handle::*;
pub use pkg_manager::*;
pub use shell_env::*;
pub use tmux::*;
pub use zsh::*;
