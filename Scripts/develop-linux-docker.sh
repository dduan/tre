#!/usr/bin/env bash

command -v docker &> /dev/null || { echo >&2 "Install docker https://www.docker.com"; exit 1; }

IMAGE=swift@sha256:dd9004042a308eccdf1d6dc960b6ad3b3006c1062eb460d2e62001c35e21f518
NAME=tredev
docker run -it -v "$PWD":/tre --name "$NAME" --rm "$IMAGE"
