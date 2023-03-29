#!/bin/bash

cd "$(dirname $0)"
./build.sh
docker push ghcr.io/rkochenderfer/gm-tools-api:latest