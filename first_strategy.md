# Build a Simple Trading Strategy with RSI and EMA

In this tutorial, you'll learn how to combine two indicators, the **Relative Strength Index (RSI)** 
and **Exponential Moving Average (EMA)** to simulate a basic buy signal.

---

## 🧠 Strategy Logic

- **Buy** when:
  - RSI drops below 30 (oversold), **and**
  - Current price is above the EMA

This is a common "momentum + trend" signal.

---

## 🚀 Step-by-Step

### Step 1: Add RustTI to your project

Make sure `rust_ti` is in your `Cargo.toml`:

```toml
[dependencies]
rust_ti = "2.1"
```

> Check [crates.io](https://crates.io/crates/rust_ti) for the latest version

---

## Step 2: Calculate the RSI and EMA

```rust
use rust_ti::moving_average::bulk::moving_average;
use rust_ti::momentum_indicators::bulk::relative_strength_index;
use rust_ti::{MovingAverageType, ConstantModelType};

[...]

let rsi = relative_strength_index(&prices, ConstantModelType::ExponentialMovingAverage, 5);
let ema = moving_average(&prices, MovingAverageType::Exponential, 5);
```

---

## Step 3: Loop and check for signals

```rust
for i in 5..prices.len() {
    let rsi_val = rsi[i - 5];
    let ema_val = ema[i - 5];
    let price = prices[i];

    if rsi_val < 30.0 && price > ema_val {
        println!("Buy signal at index {}: price={}, RSI={}, EMA={}", i, price, rsi_val, ema_val);
    }
}
```

---

## 🧪 Output

You’ll see any indices where your strategy would trigger a buy signal based on historical data

> The full code can be found at `./examples/first_strategy.rs`

---

## Next steps

- Add sell signals
- Add other indicators

