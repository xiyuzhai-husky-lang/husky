#!/bin/bash

# >>> conda initialize >>>
# !! Contents within this block are managed by 'conda init' !!
__conda_setup="$('/home/xiyuzhai/anaconda3/bin/conda' 'shell.bash' 'hook' 2> /dev/null)"
if [ $? -eq 0 ]; then
    eval "$__conda_setup"
else
    if [ -f "/home/xiyuzhai/anaconda3/etc/profile.d/conda.sh" ]; then
        . "/home/xiyuzhai/anaconda3/etc/profile.d/conda.sh"
    else
        export PATH="/home/xiyuzhai/anaconda3/bin:$PATH"
    fi  
fi
unset __conda_setup
# <<< conda initialize <<<

export PATH="$PATH:~/anaconda3/bin"
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$CONDA_PREFIX/lib/