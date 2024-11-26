# Image processing

### Rust Installation

Install [Rust](https://rustup.rs/):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Verify:

```bash
rustc --version
```

### Dependencies

Rust package manager will automatically download the dependencies.

- [`ocl`](https://crates.io/crates/ocl): OpenCL bindings and interfaces.
- [`image`](https://crates.io/crates/image): Imaging library.

### Build and Run the Program

```bash
cargo run
```
