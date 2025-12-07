# CKAD Exam Wiki

An interactive web-based wiki for Certified Kubernetes Application Developer (CKAD) exam preparation, built with Rust, Leptos, and WebAssembly.


## Prerequisites

- Rust (latest stable version)
- wasm-pack
- Python 3 (for the development server)

## Installation

1. Install Rust from [https://rustup.rs/](https://rustup.rs/)

2. Install wasm-pack:
```bash
cargo install wasm-pack
```

## Building and Running

To build and serve the application locally:

```bash
./build_and_serve.sh
```

This script will:
1. Clean the build directory
2. Build the WebAssembly package
3. Start a local server on port 8000

Once running, open your browser to:
```
http://localhost:8000
```