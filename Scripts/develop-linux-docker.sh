#!/usr/bin/env bash

command -v docker &> /dev/null || { echo >&2 "Install docker https://www.docker.com"; exit 1; }

IMAGE=swift@sha256:ccaef3f936bd3cabd184a0caf7c2455eb861182b51e77970be4be72bea116a26
NAME=tredev
docker run -it -v "$PWD":/tre --name "$NAME" --rm "$IMAGE"
