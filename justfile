set dotenv-load

export EDITOR := 'nvim'

alias f := fmt
alias t := test

default:
  just --list

ci: build test clippy fmt-check readme

[group: 'dev']
build:
  cargo build

[group: 'check']
clippy:
  cargo clippy --all-targets --all-features

[group: 'format']
fmt:
  cargo fmt

[group: 'check']
fmt-check:
  cargo fmt --all -- --check

[group: 'misc']
install:
  cargo install --path .

[group: 'release']
readme:
  present --in-place README.md

[group: 'dev']
run *args:
  cargo run {{ args }}

[group: 'dev']
test:
  cargo test

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{ COMMAND }}"
