#!/usr/bin/env bash
set -evx
docker build -t ckb-log-analyzer:v0.0.1 .
docker run -it -v ${PWD}:/app ckb-log-analyzer:v0.0.1 draw --logs-path /app/run.log --labels sample --outdir /app
docker ps -a --no-trunc
docker container prune -f
