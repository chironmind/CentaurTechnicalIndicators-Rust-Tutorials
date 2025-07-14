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
use rust_ti::ConstantModelTypei::ExponentialMovingAverage;

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

You'll see buy and sell points for the RSI based on historical data.

> The full code can be found at `./examples/backtest.rs

---

## Next steps

- Track cumulative profit
- Add stop-loss or take-profit
- Support other indicators (e.g. MACD crossovers)
