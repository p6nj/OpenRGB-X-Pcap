#!/bin/bash
# this is a setup script used only for debugging purposes, you shouldn't use it for everyday use
cargo build
sudo setcap cap_net_raw,cap_net_admin=eip target/debug/testing
cargo run
