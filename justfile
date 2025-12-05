set dotenv-load

export CARGO_MSG_LIMIT := '1'

default:
	just --list

alias f := fmt
alias r := run
alias t := test

all: build test clippy fmt-check

[group: 'dev']
build:
  cargo build

[group: 'check']
check:
 cargo check

[group: 'check']
ci: test clippy forbid
  cargo fmt --all -- --check
  cargo update --locked --package skeleton-cli

[group: 'check']
clippy:
  cargo clippy --all --all-targets

[group: 'format']
fmt:
  cargo fmt

[group: 'format']
fmt-check:
  cargo fmt --all -- --check

[group: 'check']
forbid:
  ./bin/forbid

[group: 'dev']
install:
  cargo install -f skeleton-cli

[group: 'dev']
run *args:
  cargo run {{ args }}

[group: 'test']
test:
  cargo test --all --all-targets

[group: 'dev']
watch +COMMAND='test':
  cargo watch --clear --exec "{{ COMMAND }}"
