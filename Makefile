prefix      ?= /usr/local
install_dir  = $(prefix)/bin

build:
	@swift build -c release -Xswiftc -static-stdlib -Xswiftc -warnings-as-errors --disable-sandbox

xcode:
	@swift package generate-xcodeproj

install: build
	@install ".build/release/tre" $(install_dir)

uninstall:
	@rm -rf "$(install_dir)/tre"

clean:
	@rm -rf .build
