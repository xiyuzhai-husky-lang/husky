sudo apt update
sudo apt install build-essential
sudo apt install curl
if ["$(which cargo)" = ""]; then
    curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
fi
rustup target add wasm32-unknown-unknown
if ["$(which npm)" = ""]; then
    curl -fsSL https://deb.nodesource.com/setup_18.x | sudo -E bash -
    sudo apt-get install -y nodejs
fi
sudo apt install clang
sudo apt install lld
sudo apt install pkg-config
sudo apt install libssl-dev
sudo apt install libopenblas-dev
sudo apt install mpg321
. ~/.cargo/env
cargo build
make vscode
sudo snap install scc
if ["$(which lake)" = ""]; then
    echo "needs to install lake!"
    exit 1
fi
lake build
