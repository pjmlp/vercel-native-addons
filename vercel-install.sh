#!/bin/bash
set -x
set -e
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | bash -s -- -y
export PATH="/vercel/.cargo/bin:$PATH"

echo "Install script being run"

yarn install