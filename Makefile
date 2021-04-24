# Default make target, does everything useful, and installs pre-commit hooks
all: check lint install-precommit-hooks build test deb

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

install-precommit-hooks:
	pre-commit install

.PHONY: all check lint install-precommit-hooks build test deb
