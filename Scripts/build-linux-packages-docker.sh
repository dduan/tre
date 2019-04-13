#!/usr/bin/env bash

command -v docker &> /dev/null  || { echo >&2 "Install docker https://www.docker.com"; exit 1; }

docker run -i -v ${PWD}:/tre ibmcom/swift-ubuntu:5.0 sh << COMMANDS
cd /tre; Scripts/build-linux-packages.sh
COMMANDS
