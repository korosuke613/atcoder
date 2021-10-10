#!/usr/bin/env bash

set -euxo pipefail

export YEAR=$1
export MONTH=$2
export DAY=$3

git checkout -b "asakatsu-${YEAR}${MONTH}${DAY}"
git commit --allow-empty -m "asakatsu: ${YEAR}/${MONTH}/${DAY}"
gh pr create
