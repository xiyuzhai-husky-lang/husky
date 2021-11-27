#!/bin/bash
#  Customize BASH PS1 prompt to show current GIT repository && branch.
#  by Mike Stewart - http://MediaDoneRight.com

#  SETUP CONSTANTS
#  Bunch-o-predefined colors.  Makes reading code easier than escape sequences.
#  I don't remember where I found this.  o_O

# Reset
export Reset="\033[0m"       # Text Reset

# Regular Colors
export Black="\033[0;30m"        # Black
export Red="\033[0;31m"          # Red
export Green="\033[0;32m"        # Green
export Yellow="\033[0;33m"       # Yellow
export Blue="\033[0;34m"         # Blue
export Purple="\033[0;35m"       # Purple
export Cyan="\033[0;36m"         # Cyan
export White="\033[0;37m"        # White

# Bold
export BBlack="\033[1;30m"       # Black
export BRed="\033[1;31m"         # Red
export BGreen="\033[1;32m"       # Green
export BYellow="\033[1;33m"      # Yellow
export BBlue="\033[1;34m"        # Blue
export BPurple="\033[1;35m"      # Purple
export BCyan="\033[1;36m"        # Cyan
export BWhite="\033[1;37m"       # White

# Underline
export UBlack="\033[4;30m"       # Black
export URed="\033[4;31m"         # Red
export UGreen="\033[4;32m"       # Green
export UYellow="\033[4;33m"      # Yellow
export UBlue="\033[4;34m"        # Blue
export UPurple="\033[4;35m"      # Purple
export UCyan="\033[4;36m"        # Cyan
export UWhite="\033[4;37m"       # White

# Background
export On_Black="\033[40m"       # Black
export On_Red="\033[41m"         # Red
export On_Green="\033[42m"       # Green
export On_Yellow="\033[43m"      # Yellow
export On_Blue="\033[44m"        # Blue
export On_Purple="\033[45m"      # Purple
export On_Cyan="\033[46m"        # Cyan
export On_White="\033[47m"       # White

# High Intensty
export IBlack="\033[0;90m"       # Black
export IRed="\033[0;91m"         # Red
export IGreen="\033[0;92m"       # Green
export IYellow="\033[0;93m"      # Yellow
export IBlue="\033[0;94m"        # Blue
export IPurple="\033[0;95m"      # Purple
export ICyan="\033[0;96m"        # Cyan
export IWhite="\033[0;97m"       # White

# Bold High Intensty
export BIBlack="\033[1;90m"      # Black
export BIRed="\033[1;91m"        # Red
export BIGreen="\033[1;92m"      # Green
export BIYellow="\033[1;93m"     # Yellow
export BIBlue="\033[1;94m"       # Blue
export BIPurple="\033[1;95m"     # Purple
export BICyan="\033[1;96m"       # Cyan
export BIWhite="\033[1;97m"      # White

# High Intensty backgrounds
export On_IBlack="\033[0;100m"   # Black
export On_IRed="\033[0;101m"     # Red
export On_IGreen="\033[0;102m"   # Green
export On_IYellow="\033[0;103m"  # Yellow
export On_IBlue="\033[0;104m"    # Blue
export On_IPurple="\033[10;95m"  # Purple
export On_ICyan="\033[0;106m"    # Cyan
export On_IWhite="\033[0;107m"   # White
if [[ `uname` == "Darwin" ]]; then
echo -e "${Purple}Environment: Macbook${RESET}"
export HUSKY_ROOT=/Users/xiyuzhai/Documents/husky
export HUSKY_USER_ROOT=/Users/xiyuzhai/.husky
export CXX="clang++ -fdiagnostics-absolute-paths -g -Werror -Wfatal-errors -ferror-limit=1"
else
echo $HOST
echo "Environment: Alienware"
export HUSKY_ROOT=/home/xiyuzhai/Documents/husky
export HUSKY_USER_ROOT=/home/xiyuzhai/.husky
export CXX="clang++ -fdiagnostics-absolute-paths -g -Werror -Wfatal-errors -ferror-limit=1"
fi
alias b="$HUSKY_ROOT/core/scripts/build_core.sh"
alias r="$HUSKY_ROOT/core/scripts/run.sh"
alias c="$HUSKY_ROOT/compile.sh"
alias h="$HUSKY_ROOT/core_haskell/scripts/compile.sh && $HUSKY_ROOT/core_haskell/main"
alias t="c"
alias tt="t"
alias ttt="t"
alias g="cd $HUSKY_ROOT/projects/mnist/.cpp_debug/build && gdb mnist"
alias d="$HUSKY_ROOT/core/scripts/doc.sh"
alias count="cloc --exclude-dir=out,node_modules,build,build-ninja,build-meson,package-lock.json,package.json,docs,projects,.old,_outdated,cpp_core --skip-uniqueness $HUSKY_ROOT"
alias cloc_all="for file in *; do echo -e \$Red\$file\$Reset; cloc \$file; done"

# Various variables you might want for your PS1 prompt instead
#Time12h="\T"
#Time12a="\@"
#PathShort="\w"
#PathFull="\W"
#NewLine="\n"
#Jobs="\j"


# This PS1 snippet was adopted from code for MAC/BSD I saw from: http://allancraig.net/index.php?option=com_content&view=article&id=108:ps1-export-command-for-git&catid=45:general&Itemid=96
# I tweaked it to work on UBUNTU 11.04 & 11.10 plus made it mo' better

#export PS1=$IBlack$Time12h$Color_Off'$(git branch &>/dev/null;\
#if [ $? -eq 0 ]; then \
#  echo "$(echo `git status` | grep "nothing to commit" > /dev/null 2>&1; \
#  if [ "$?" -eq "0" ]; then \
#    # @4 - Clean repository - nothing to commit
#    echo "'$Green'"$(__git_ps1 " (%s)"); \
#  else \
#    # @5 - Changes to working tree
#    echo "'$IRed'"$(__git_ps1 " {%s}"); \
#  fi) '$BYellow$PathShort$Color_Off'\$ "; \
#else \
#  # @2 - Prompt when not in GIT repo
#  echo " '$Yellow$PathShort$Color_Off'\$ "; \
#fi)'

# enable this flag ONLY if you are working with an internal repository that doesn't have a valid certificate
# export GIT_SSL_NO_VERIFY=true

#git config --global alias.lg "log --color --graph --pretty=format:'%Cred%h%Creset -%C(yellow)%d%Creset %s %Cgreen(%cr)%C(bold blue)<%an>%Creset'
