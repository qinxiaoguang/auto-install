# 配置环境变量，都放在bashrc中

# 配置别名 ====== start
## git
alias gp="git branch | grep \"*\" | awk '{print \"git push origin HEAD:refs/for/\"\$2 | \"/bin/bash\"}'"
alias gcn="(git pull || echo 0)&& git branch -r|tail -2 | head -1 | sed -e 's/origin\///g' | awk '{print \"git checkout \"\$1 | \"/bin/bash\"}'"
alias gpull="git pull"
alias gl="git log"
alias gc="git checkout"
alias gs="git status"
alias gm="git commit -m"
alias gma="git commit --amend"
## other
alias tailf="tail -f"
# 配置别名 ====== end

# 配置rust
export CARGO_HOME="$HOME/.cargo"
export RUSTBINPATH="$CARGO_HOME/bin"
export RUST="$HOME/.rustup/toolchains/stable-x86_64-apple-darwin"
export RUST_SRC_PATH="$RUST/lib/rustlib/src/rust/src"
# 清华大学
export RUSTUP_DIST_SERVER=https://mirrors.tuna.tsinghua.edu.cn/rustup
# 中国科学技术大学
#export RUSTUP_DIST_SERVER=https://mirrors.ustc.edu.cn/rust-static
#export RUSTUP_UPDATE_ROOT=https://mirrors.ustc.edu.cn/rust-static/rustup
# 上海交通大学
#export RUSTUP_DIST_SERVER=https://mirrors.sjtug.sjtu.edu.cn/rust-static/
export PATH=$PATH:$RUSTBINPATH

# brew镜像 中科大
HOMEBREW_CORE_GIT_REMOTE=https://mirrors.ustc.edu.cn/homebrew-core.git
