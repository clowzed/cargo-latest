# Cargo latest

## About

Tired of going to `crates.io` for searching last stable version of a crate?
Here is a CLI tool for `making your life` a bit better.

## Installation

```bash
git clone https://github.com/clowzed/cargo-latest.git
cd cargo-latest
cargo build --release
# Add /target/release/cargo-latest.exe to PATH
```

## Usage
```bash
cargo-latest -c actix
cargo-latest -c actix --latest false
cargo-latest --crate_name actix --latest true
```

## Arguments
|short| long| description| default|
|--|--|--|--|
|-c|--crate_name|Sets crate name|--|
|-l|--latest| Set it to false if you want all versions|true|


## TODO
- [ ] Upload crate to `crates.io`