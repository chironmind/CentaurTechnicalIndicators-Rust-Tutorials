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

```shell
$ cargo run --example first_strategy

Buy signal at index 9: price=5204.34, RSI=10.370911367310825, EMA=5192.766398104266
Buy signal at index 20: price=5010.6, RSI=0, EMA=5004.590663507109
Buy signal at index 44: price=5304.72, RSI=25.310005095973736, EMA=5294.27383886256
Buy signal at index 48: price=5277.51, RSI=23.42549042101595, EMA=5265.902085308057
Buy signal at index 51: price=5354.03, RSI=27.976852154580726, EMA=5278.745118483413
Buy signal at index 68: price=5475.09, RSI=27.183608994610452, EMA=5469.229289099527
Buy signal at index 135: price=5751.07, RSI=24.429997252998234, EMA=5713.912511848342
Buy signal at index 167: price=5916.98, RSI=20.56683710737768, EMA=5914.501327014219
Buy signal at index 168: price=5917.11, RSI=25.50556255822852, EMA=5911.9400000000005
Buy signal at index 169: price=5948.71, RSI=11.327387959812228, EMA=5910.212085308057
Buy signal at index 197: price=5942.47, RSI=0, EMA=5902.900521327015
Buy signal at index 204: price=5949.91, RSI=7.843420273956696, EMA=5852.073507109006
```

> The full code can be found at [`./examples/first_strategy.rs`](./examples/first_strategy.rs)

---

## Next steps

- Add sell signals
- Add other indicators
- Explore the [how-to guides](https://github.com/ChironMind/RustTI-how-to-guides) to update period and models
