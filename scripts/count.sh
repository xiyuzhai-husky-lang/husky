#!/bin/bash
# cloc ./ --exclude-dir=__rust_gen__,__rust_gen_cache__,node_modules,out,target,dist,forks,package-lock.json,package.json --exclude-ext=html,css,json,toml
scc ./ --exclude-dir=expect-files,tests,__rust_gen__,__rust_gen_cache__,node_modules,out,target,dist,forks,package-lock.json,package.json,__rust_code_gen__.rs,sparks,static
