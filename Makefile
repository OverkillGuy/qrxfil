
all: check lint build test deb

check:
	cargo check

lint:
	pre-commit run --all --all-files

build:
	cargo build

test:
	cargo test

deb:
	cargo deb

.PHONY: all check lint build test deb
