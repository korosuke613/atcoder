#!/usr/bin/env bash

# argument example: https://atcoder.jp/contests/abs/tasks/practice_1
allow_dir=contests
deno run \
  --unstable \
  --allow-read="./" \
  --allow-write="${allow_dir}" \
  tools/setup.ts "$@"
