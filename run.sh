#!/bin/bash

export NODE_ENV=local

cargo build -p users
cargo build -p leagues
cargo build -p teams
cargo build -p players

cargo run -p users & cargo run -p leagues & cargo run -p teams & cargo run -p players & sleep 5; npm run start-gateway --prefix gateway/
