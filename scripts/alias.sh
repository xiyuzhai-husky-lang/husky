#!/bin/bash
alias install="scripts/install_server.sh"
alias test="cargo test"
alias t=install
alias tt="t"
alias ttt="t"
alias find_duplicate="find crates ! -empty -type f -exec md5sum {} + | sort | uniq -w32 -dD"
