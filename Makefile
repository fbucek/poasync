.PHONY: clean check test build doc publish

all: clean check test build doc

check:
	cargo update
	cargo check
	cargo clippy
	cargo fmt

test: 
	cargo test

build:
	cargo build

doc:
	cargo doc --no-deps --document-private-items --open

clean: 
	cargo clean --doc

# Publishing to crates
publish: all
	cargo publish
