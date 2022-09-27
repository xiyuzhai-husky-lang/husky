#!/bin/bash
if [[ $(which cargo) == "" ]]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
if [[ $(which zld) == "" ]]; then
    echo "needs zld"
    exit 1
fi
if [[ $(which npm) == "" ]]; then
    echo "needs npm"
    exit 1
fi
