#!/usr/bin/env just --justfile
set dotenv-load := true

# interactive menu by default
default:
  @just --choose

# outputs list of tasks
list:
  @just --list

# apply strict formatting
fmt *FLAGS:
  cargo +nightly fmt --all {{FLAGS}}

# run test suite
test *FLAGS:
  cargo nextest run --all-features --workspace {{FLAGS}}

# Benchmark codebase with criterion.
benchmark *FLAGS:
  cargo criterion {{FLAGS}}

# generate documentation
doc:
  mdbook build

# setup dev environment
init:
  echo # installing nightly used by `just fmt` and `cargo udeps`
  rustup install nightly

  echo # installing cargo-binstall for faster setup time
  cargo binstall -V || cargo install cargo-binstall

  echo # requirements for `just test`
  cargo binstall cargo-nextest --no-confirm

  echo # requirements for `just doc`
  cargo binstall mdbook --no-confirm
  cargo binstall mdbook-admonish --no-confirm
  cargo binstall mdbook-mermaid --no-confirm

  echo # requirements for `just benchmark`
  cargo binstall cargo-criterion --no-confirm

  echo # requirements for `just thorough-check`
  cargo binstall cargo-udeps --no-confirm
  cargo binstall cargo-audit --no-confirm
  cargo binstall cargo-upgrades --no-confirm
  cargo binstall cargo-unused-features --no-confirm
