#!/usr/bin/env bash

bin_name=$(deno run --unstable tools/run.ts "$@")
cargo run --manifest-path contests/Cargo.toml --bin "${bin_name}"
