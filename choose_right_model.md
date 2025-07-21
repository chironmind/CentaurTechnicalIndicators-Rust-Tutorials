# Choosing the right Constant Model Type for the RSI

In this tutorial, you'll learn how to choose the right **Constant Type Model** for the **Relative Strength Index (RSI)** .

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

## Step 2: Calculate the different RSI 

```rust
use rust_ti::momentum_indicators::bulk::relative_strength_index;
use rust_ti::ConstantModelType;

[...]

let rsi_ma = relative_strength_index(&prices, ConstantModelType::SimpleMovingAverage, 5);
let rsi_sma = relative_strength_index(&prices, ConstantModelType::SmoothedMovingAverage, 5);
let rsi_ema = relative_strength_index(&prices, ConstantModelType::ExponentialMovingAverage, 5);
```

---

## Step 3: Rate the different indicators

This rating algorithm is overlysimplified, and is intended for demonstration purposes only.

```rust

let mut rsi_ma_rating = 0;
let mut rsi_sma_rating = 0;
let mut rsi_ema_rating = 0;

for i in 5..prices.len()-1 {
    let rsi_ma_val = rsi_ma[i - 5];
    let rsi_sma_val = rsi_sma[i - 5];
    let rsi_ema_val = rsi_ema[i - 5];
    let price = prices[i];

    if rsi_ma_val < 30.0 {
        println!("Buy signal at index {}: price={}, RSI(MA)={}", i, price, rsi_ma_val);
        
        if prices[i+1] > prices[i] {
            println!("Signal was correct");
            rsi_ma_rating += 1;
        } else {
            println!("Signal was incorrect");
        }
    }
    
    if rsi_sma_val < 30.0 {
        println!("Buy signal at index {}: price={}, RSI(SMA)={}", i, price, rsi_sma_val);
     

        if prices[i+1] > prices[i] {
            println!("Signal was correct");
            rsi_sma_rating += 1;
        } else {
            println!("Signal was incorrect");
        };
    }

    if rsi_ema_val < 30.0 {
        println!("Buy signal at index {}: price={}, RSI(EMA)={}", i, price, rsi_ema_val);
     
        if prices[i+1] > prices[i] {
            println!("Signal was correct");
            rsi_ema_rating += 1;
        } else {
            println!("Signal was incorrect");
        };
    }
}

if rsi_ma_rating > rsi_sma_rating && rsi_ma_rating > rsi_ema_rating {
    println!("Moving Average is the best model")
} else if rsi_sma_rating > rsi_ema_rating && rsi_sma_rating > rsi_ma_rating {
    println!("Smoothed Moving Average is the best model")
} else if rsi_ema_rating > rsi_ma_rating && rsi_ema_rating > rsi_sma_rating {
    println!("Exponential Moving Average is the best model")
} else {
    println!("No clear winnder")
}

```

---

## 🧪 Output

```shell
$ cargo run --example choose_right_model

Buy signal at index 8: price=5147.21, RSI(MA)=19.20772303595203
Signal was correct
Buy signal at index 8: price=5147.21, RSI(SMA)=16.602391052834022
Signal was correct
Buy signal at index 8: price=5147.21, RSI(EMA)=15.540173724212082
Signal was correct
[...]
Buy signal at index 230: price=5983.25, RSI(MA)=18.387940700136852
Signal was incorrect
Buy signal at index 230: price=5983.25, RSI(SMA)=15.774597495527516
Signal was incorrect
Buy signal at index 230: price=5983.25, RSI(EMA)=14.72167846207877
Signal was incorrect
Exponential Moving Average is the best model
```

> The full code can be found at [`./examples/choose_right_model.rs`](./examples/choose_right_model.rs)

---

## Next steps

- Improve rating algorithm by adding punishment
- Add other variants of `ConstantTypeModel`
- Try with different indicators


