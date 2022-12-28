#!/bin/bash
sudo rm /usr/local/bin/wxr
cargo build --release
sudo cp target/release/wxr /usr/local/bin/wxr