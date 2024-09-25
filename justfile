#!/usr/bin/env just --justfile

debug:
  cargo build

release:
  cargo build --release    

lint:
  cargo clippy

bin:
  cargo run --bin bin -- arg1

example:
  cargo run --example exname -- arg1


test:
  cargo build
  php -d extension=./target/debug/libsyntext_ext_php.dylib test/test.php

install-macos:
  just release
  cp ./target/release/libsyntext_ext_php.dylib /opt/homebrew/lib/php/pecl/20230831
  nvim /opt/homebrew/etc/php/8.3/php.ini