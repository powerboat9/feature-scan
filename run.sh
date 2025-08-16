#!/bin/sh
set -e

RUST_VERSION=1.49

DOCKER_DIR="$(dirname "$0")"

cat "$DOCKER_DIR"/Dockerfile.tpl | \
  sed 's/RUST_VERSION/'"$RUST_VERSION"'/g' > \
  "$DOCKER_DIR"/Dockerfile

docker build -t feature-scan-container-$RUST_VERSION -- "$DOCKER_DIR"
docker run --rm feature-scan-container-$RUST_VERSION
