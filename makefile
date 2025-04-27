# Makefile for Jito Restaking Project

# Commands
CARGO := cargo
CARGO_SORT := cargo sort
CARGO_CLIPPY := cargo clippy
CARGO_FMT := cargo +nightly fmt
CARGO_SBF := cargo-build-sbf
CARGO_NEXTEST := cargo nextest
PNPM := pnpm
SHANK_CLI := ./target/release/shank-cli
RESTAKING_CLI := ./target/release/jito-restaking-cli

# Default target
.PHONY: all
all: build test

# Linting
.PHONY: lint
lint:
	$(CARGO_SORT) --workspace --check
	$(CARGO_FMT) --all --check
	$(CARGO_CLIPPY) --all-features -- -D warnings -D clippy::all -D clippy::nursery -D clippy::integer_division -D clippy::arithmetic_side_effects -D clippy::style -D clippy::perf

# Code generation
.PHONY: generate-code
generate-code: build-release
	$(SHANK_CLI)
	$(PNPM) install
	$(PNPM) generate-clients

# Build debug
.PHONY: build
build:
	$(CARGO) build

# Build release
.PHONY: build-release
build-release:
	$(CARGO) build --release

# Build Solana BPF/SBF programs
.PHONY: build-sbf
build-sbf:
	$(CARGO_SBF)

# Run tests
.PHONY: test
test:
	$(CARGO_SBF)
	SBF_OUT_DIR=$(shell pwd)/target/sbf-solana-solana/release $(CARGO_NEXTEST) run --all-features

# Format code
.PHONY: format
format:
	$(CARGO_SORT) --workspace
	$(CARGO_FMT) --all
