#!/bin/bash
export LIBTORCH=$HOME/.cpp-dependencies/libtorch-cxx11-abi-shared-with-deps-2.0.1+cu117/libtorch
export LD_LIBRARY_PATH=$LD_LIBRARY_PATH:$LIBTORCH/lib
sudo apt-get install libopenblas-dev
sudo apt-get install python3-dev