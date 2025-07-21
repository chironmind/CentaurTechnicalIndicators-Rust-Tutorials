# Getting Started with RustTI

Welcome! This tutorial will guide you through setting up a Rust project using [RustTI](https://crates.io/crates/rust_ti), a high-performance technical indicator library.

## Prerequisites

- Rust and Cargo installed: https://rust-lang.org/tools/install
- A terminal and your favorite editor

---

## Step 1: Create a new project

```bash
cargo new rustti-demo
cd rustti-demo
```

---

## Step 2: Add RustTI as a dependency

In your `Cargo.toml` add:

```toml
[dependencies]
rust_ti = "2.1"
```
> Check [crates.io](https://crates.io/crates/rust_ti) for the latest version

---

## Step 3: Calculate your first indicator

Replace the contents of `src/main.rs` with:

```rust
use rust_ti::moving_average::bulk::moving_average;
use rust_ti::MovingAverageType::Simple;

fn main() {
    let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10];
    let period: usize = 5;
    let result = moving_average(&prices, Simple, period);

    println!("Simple MA values: {:?}", result);
}

```

Then run:

```bash
cargo run
```

---

## 🧪 Output

```shell
$ cargo run --example getting_started

Simple MA values: [44.104, 44.202, 44.40399999999999]
```

> The full code can be found at [`./examples/getting_started.rs`](./examples/getting_started.rs)

---

## Next steps

- Try other indicators
- Check out other tutorials in this repo

