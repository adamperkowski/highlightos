#! /bin/bash

rustup override set nightly
cargo install cargo-binutils
rustup component add llvm-tools-preview
