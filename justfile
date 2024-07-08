default:
  just --list

all: build test clippy fmt-check readme

build:
  cargo build

clippy:
  cargo clippy --all-targets --all-features

fmt:
  cargo fmt

fmt-check:
  cargo fmt --all -- --check
  @echo formatting check done

install:
  cargo install --path .

readme:
  present --in-place README.md

run *args:
  cargo run -- {{args}}

test:
  cargo test

watch +COMMAND='test':
  cargo watch --clear --exec "{{COMMAND}}"
