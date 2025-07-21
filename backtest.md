# Run a Simple Backtest Loop

In this tutorial, you'll build a minimal backtester that simulates a basic strategy using **RSI** to enter and exit trades. This helps you understand how strategy logic behaves over time.

---

## 💡 Strategy Rules

- **Enter trade** when RSI < 30 (oversold)
- **Exit trade** when RSI > 70 (overbought)
- Only one open position at a time
- Track and log entry/exit and P&L

---

## Step 1: Setup

Ensure `rust_ti` is in your `Cargo.toml`:

```toml
[dependencies]
rust_ti = "2.1"
```

---

## Step 2: Calculate RSI

```rust 

use rust_ti::momentum_indicators::bulk::relative_strength_index;
use rust_ti::ConstantModelType::ExponentialMovingAverage;

[...]

let rsi = relative_strength_index(&prices, ExponentialMovingAverage, 5); 

```

---

## Step 3: Backtest loop

```rust

let mut in_position = false;
let mut entry_price = 0.0;

for i in 5..prices.len() {
    let price = prices[i];
    let rsi_val = rsi[i - 5];

    if !in_position && rsi_val < 30.0 {
        in_position = true;
        entry_price = price;
        println!("BUY at index {}: price = {:.2}, RSI = {:.2}", i, price, rsi_val);
    } else if in_position && rsi_val > 70.0 {
        in_position = false;
        let profit = price - entry_price;
        println!("SELL at index {}: price = {:.2}, RSI = {:.2}, Profit = {:.2}", i, price, rsi_val, profit);
    }
}

```

---

## 🧪 Output

```shell
$ cargo run --example backtest

BUY at index 8: price = 5147.21, RSI = 15.54
SELL at index 11: price = 5209.91, RSI = 71.63, Profit = 62.70
BUY at index 18: price = 5011.12, RSI = 0.00
SELL at index 31: price = 5187.70, RSI = 76.06, Profit = 176.58
BUY at index 44: price = 5304.72, RSI = 25.31
SELL at index 52: price = 5352.96, RSI = 100.00, Profit = 48.24
BUY at index 68: price = 5475.09, RSI = 27.18
SELL at index 72: price = 5572.85, RSI = 100.00, Profit = 97.76
BUY at index 80: price = 5544.59, RSI = 27.20
SELL at index 84: price = 5427.13, RSI = 72.53, Profit = -117.46
BUY at index 88: price = 5436.44, RSI = 25.64
SELL at index 99: price = 5455.21, RSI = 100.00, Profit = 18.77
BUY at index 116: price = 5471.05, RSI = 0.00
SELL at index 120: price = 5626.02, RSI = 100.00, Profit = 154.97
BUY at index 134: price = 5699.94, RSI = 13.68
SELL at index 141: price = 5859.85, RSI = 76.74, Profit = 159.91
BUY at index 150: price = 5808.12, RSI = 27.34
SELL at index 153: price = 5813.67, RSI = 86.91, Profit = 5.55
BUY at index 155: price = 5728.80, RSI = 11.25
SELL at index 159: price = 5973.10, RSI = 86.91, Profit = 244.30
BUY at index 164: price = 5949.17, RSI = 24.59
SELL at index 170: price = 5969.34, RSI = 100.00, Profit = 20.17
BUY at index 188: price = 5867.08, RSI = 17.46
SELL at index 192: price = 6037.59, RSI = 92.09, Profit = 170.51
BUY at index 196: price = 5868.55, RSI = 0.00
SELL at index 198: price = 5975.38, RSI = 75.62, Profit = 106.83
BUY at index 201: price = 5827.04, RSI = 27.55
SELL at index 206: price = 5996.66, RSI = 83.66, Profit = 169.62
BUY at index 223: price = 6051.97, RSI = 21.63
SELL at index 225: price = 6114.63, RSI = 71.96, Profit = 62.66
BUY at index 230: price = 5983.25, RSI = 14.72

```

> The full code can be found at [`./examples/backtest.rs`](./examples/backtest.rs)

---

## Next steps

- Track cumulative profit
- Add stop-loss or take-profit
- Support other indicators (e.g. MACD crossovers)
