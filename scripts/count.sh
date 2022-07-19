#!/bin/bash
cloc ./ --exclude-dir=.compiled,node_modules,out,target,dist,forks,package-lock.json,package.json --exclude-ext=html,css,json,toml
