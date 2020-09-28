
.PHONY: check
check:
	@cargo check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

setup:
	@rustup default stable
	@rustup component add rls rust-analysis rust-src
