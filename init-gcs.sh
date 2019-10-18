#!/bin/bash
# Run this shell script to install Rust in a Google Cloud Shell environment
# sudo apt-get update && sudo apt-get -y upgrade
curl https://sh.rustup.rs -sSf | sh
source $HOME/.cargo/env
