# Visualize Your Indicators with Plotters

In this tutorial, you’ll learn how to plot prices along with technical indicators using [`plotters`](https://crates.io/crates/plotters), a flexible Rust drawing library.

---

## Step 1: Setup

In `Cargo.toml`:

```toml
[dependencies]
rust_ti = "2.1"
plotters = "0.3"
```

---

## Step 2: Calculate Simple Moving Average

```rust

use rust_ti::moving_average::bulk::moving_average;
use rust_ti::MovingAverageType::Simple;

[...]

let ma = moving_average(&prices, Simple, 5);

```

---

## Step 3: Create line chart with Plotters

```rust

use plotters::prelude::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {

    [...]

    let root = BitMapBackend::new("ma_chart.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let max_price = prices.iter().cloned().fold(f64::MIN, f64::max);
    let min_price = prices.iter().cloned().fold(f64::MAX, f64::min);

    let mut chart = ChartBuilder::on(&root)
        .caption("Price and MA", ("sans-serif", 30))
        .margin(20)
        .x_label_area_size(30)
        .y_label_area_size(40)
        .build_cartesian_2d(0..prices.len(), min_price..max_price)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        prices.iter().enumerate().map(|(i, p)| (i, *p)),
        &BLUE,
    ))?.label("Price").legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &BLUE));

    chart.draw_series(LineSeries::new(
        ma.iter().enumerate().map(|(i, s)| (i + 5, *s)), // offset for MA
        &RED,
    ))?.label("MA").legend(|(x, y)| Path::new(vec![(x, y), (x + 20, y)], &RED));

    chart.configure_series_labels().background_style(&WHITE).draw()?;

    Ok(())
}

```

---

## 🧪 Output

You’ll see the price and MA overlaid in different colors

![MA Chart Example](./examples/ma_chart.png)

> The full code can be found at [`./examples/visualization.rs`](./examples/visualization.rs)

---

## Next steps

- Add RSI or MACD as overlays
- Try candlestick charts with OHLC data
- Explore interactive plots using egui or iced
