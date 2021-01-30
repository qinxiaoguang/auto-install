use async_std::process::Command;
use std::process::Command as Cmd;

// 连接
pub fn ln(args: &[&str]) -> std::io::Result<std::process::Output> {
    Cmd::new("ln").args(args).output()
}

// 复制
pub fn cp(args: &[&str]) -> std::io::Result<std::process::Output> {
    Cmd::new("cp").args(args).output()
}
