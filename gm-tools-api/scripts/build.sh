#!/bin/bash

cd "$(dirname $0)"
cd ../

docker build -t ghcr.io/rkochenderfer/gm-tools-api:latest . --load