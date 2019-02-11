prefix      ?= /usr/local
install_dir  = $(prefix)/bin

build: clean
ifeq ($(shell uname -s), Darwin)
	@swift build -c release -Xswiftc -static-stdlib -Xswiftc -warnings-as-errors --disable-sandbox
else
	@swift build -c release -Xswiftc -warnings-as-errors --disable-sandbox
endif

xcode:
	@swift package generate-xcodeproj

install: build
	@install ".build/release/tre" $(install_dir)

uninstall:
	@rm -rf "$(install_dir)/tre"

clean:
	@rm -rf .build

test: clean build integration-test

integration-test: build
	@Scripts/integration-tests.py

develop-docker:
	@Scripts/develop-linux-docker.sh

test-docker:
	@Scripts/run-tests-linux-docker.sh

