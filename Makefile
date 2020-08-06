
.PHONY: check
check:
	@cargo check

.PHONY: build
build:
	@cargo build

.PHONY: test
test:
	@cargo test

.PHONY: check-cargo-lock
check-cargo-lock:
	@rm -rf Cargo.lock
	@cargo check

setup:
	@rustup default stable
	@rustup component add rls rust-analysis rust-src
