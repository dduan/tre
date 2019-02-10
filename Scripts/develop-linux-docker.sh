#!/usr/bin/env bash

command -v docker &> /dev/null || { echo >&2 "Install docker https://www.docker.com"; exit 1; }

IMAGE=swift@sha256:501824b412386aa7ba6d08cd8029b5fdda9b6a1e16d688d910cabdbb371ccb51
NAME=tredev
docker run -it -v "$PWD":/tre --name "$NAME" --rm "$IMAGE"
