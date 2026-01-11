# echor 🦀

`echor` is a simple Rust-based CLI utility that mimics basic behavior of the Unix `echo` command.  
It prints text to standard output, with an option to omit the trailing newline.

This project is mainly intended as a **learning exercise for Rust CLI development using `clap`**.

---

## Features

- Print one or more words to stdout
- Supports multiple input arguments
- Optional `-n` flag to omit the trailing newline
- Built using Rust and the `clap` crate

---

## Installation

### Prerequisites

- Rust (stable)
- Cargo (comes with Rust)

Check installation:

```bash
rustc --version
cargo --version
```

## Clone
```bash
git clone https://github.com/iamkashifyousuf/echor.git
cd echor
cargo build --release
````

## Copy the Binary into $PATH
```bash
sudo cp target/release/echor /usr/local/bin/
```

## Usage
```bash
echor <Text>
```