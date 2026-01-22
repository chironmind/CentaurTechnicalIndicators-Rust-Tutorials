# Getting Started with Centaur Technical Indicators

Welcome! This tutorial will guide you through setting up a Rust project using [Centaur Technical Indicators](https://crates.io/crates/centaur_technical_indicators), a high-performance technical indicator library.

## Prerequisites

- Rust and Cargo installed: https://rust-lang.org/tools/install
- A terminal and your favorite editor

---

## Step 1: Create a new project

```bash
cargo new centaur-demo
cd centaur-demo
```

---

## Step 2: Add Centaur Technical Indicators as a dependency

In your `Cargo.toml` add:

```toml
[dependencies]
centaur_technical_indicators = "1.0"
```
> Check [crates.io](https://crates.io/crates/centaur_technical_indicators) for the latest version

---

## Step 3: Calculate your first indicator

Replace the contents of `src/main.rs` with:

```rust
use centaur_technical_indicators::moving_average::bulk::moving_average;
use centaur_technical_indicators::MovingAverageType::Simple;

fn main() {
    let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10];
    let period: usize = 5;
    let result = moving_average(&prices, Simple, period).unwrap();

    println!("Simple MA values: {:?}", result);
}

```

Then run:

```bash
cargo run
```

---

## 🧪 Output

> A full runnable example can be found at [`./examples/getting_started.rs`](./examples/getting_started.rs)

Run:
```shell
cargo run --example getting_started
```
Expected output:
```shell
Simple MA values: Ok([44.104, 44.202, 44.40399999999999])
```

---

## Next steps

- Try other indicators
- [Choosing the right model](https://github.com/ChironMind/CentaurTechnicalIndicators-Rust-Tutorials/blob/main/choose_right_model.md) - An introduction to picking the right model for your indicators

