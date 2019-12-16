.PHONY: build
build:
	@cargo build --release
	@mv target/release/tre-command target/release-tre

.PHONY: check
check:
	@cargo check

.PHONY: test
test:
	@cargo test

setup:
	@rustup default stable
	@rustup component add rls rust-analysis rust-src
