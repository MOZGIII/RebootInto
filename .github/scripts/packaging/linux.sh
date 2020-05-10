#!/bin/bash
set -euo pipefail

ARTIFACTS_DIR="$1"

gbp buildpackage --git-ignore-branch

mkdir -p "$ARTIFACTS_DIR"

ARTIFACTS=(
  ../*.deb
)

cp -t "$ARTIFACTS_DIR" "${ARTIFACTS[@]}"
