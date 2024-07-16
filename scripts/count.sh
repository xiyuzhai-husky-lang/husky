#!/bin/bash
git pull
# cloc crates --exclude-dir=node_modules,out,target,dist,forks,package-lock.json,package.json,target-rs --exclude-ext=html,css,json,toml,md
scc ./ --exclude-dir=expect-files,tests,target-rs,__rust_gen__,__rust_gen_cache__,node_modules,out,target,dist,forks,package-lock.json,package.json,target-rs
