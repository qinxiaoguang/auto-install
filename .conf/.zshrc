# 所有环境变量都放在bashrc中，保证无论是bashrc还是zshrc，bashrc中的环境变量都会被source
# 而zshrc则放置与zsh有关的变量
export HOMEBREW_BOTTLE_DOMAIN=https://mirrors.ustc.edu.cn/homebrew-bottles

plugins=(
  git
  zsh-autosuggestions
  zsh-syntax-highlighting
  autojump
)

ZSH_THEME="ys"
source $ZSH/oh-my-zsh.sh

# bashrc统一输出变量, 一定要在ohmyzsh之后，否则gp等alias将被覆盖
source ~/.bashrc
# 颜色输出一定要在oh-my-zsh.sh之后
#export CLICOLOR=1                       # 是否输出颜色
#export LSCOLORS='CxfxcxdxbxegedabagGxGx'    # 指定颜色