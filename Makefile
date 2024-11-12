# Makefile for email_pest_parser

run:
	cargo run $(args)

build:
	cargo build	

test:
	cargo test

format:
	cargo fmt

clippy:
	cargo clippy -- -D warnings

prepare:
	cargo fmt && cargo clippy && cargo test

all: fmt clippy test build