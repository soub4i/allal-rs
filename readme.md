# Allal-rs CLI

A simple Rust-based CLI tool that generates an ASCII art portrait of Allal Alkadouss (a wise man) with customizable speech bubbles.

## Synopsis

```bash
allal-rs "Your message here"
```

Example output:
```
                 ____________
                /            \
               |  Your message|
                \____________/
                      \
      ____//____       \
     /    ||    \
    |  (~°||°~)  |
     \    ||    /
      \   ||   /
       \__||__/
       |-----|
      /|     |\
     / |     | \
    /  |     |  \
       |     |
       \_____/
```

## Installation

### Using curl (Linux/macOS)

```bash
curl -L https://github.com/soub4i/allal-rs/releases/latest/download/allal-rs-x86_64-unknown-linux-gnu.tar.gz | tar xz -C /usr/local/bin

# For macOS
curl -L https://github.com/soub4i/allal-rs/releases/latest/download/allal-rs-x86_64-apple-darwin.tar.gz | tar xz -C /usr/local/bin

# Make it executable
chmod +x /usr/local/bin/allal-rs
```

### From Cargo

```bash
cargo install allal-rs
```

### From Source

1. Clone the repository
```bash
git clone https://github.com/soub4i/allal-rs.git
cd allal-rs
```

2. Build and install
```bash
cargo build --release
cargo install --path .
```

## Usage

Basic usage with default message:
```bash
allal-rs
```

Custom message:
```bash
allal-rs "Wisdom comes to those who wait"
```



## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.



