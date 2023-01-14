.DEFAULT_GOAL := all

.PHONY: build-prod
build-prod:
	cargo build --release

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo fmt --version
	cargo fmt --all -- --check
	cargo clippy --version
	cargo clippy -- -D warnings -A incomplete_features
	cargo doc

.PHONY: test
test:
	cargo test --test parse && cargo test --test server

.PHONY: all
all: format lint test