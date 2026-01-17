use centaur_technical_indicators::moving_average::bulk::moving_average;
use centaur_technical_indicators::MovingAverageType::Simple;

fn main() {
    let prices = vec![44.34, 44.09, 44.15, 43.61, 44.33, 44.83, 45.10];
    let period: usize = 5;
    let result = moving_average(&prices, Simple, period);

    println!("Simple MA values: {:?}", result);
}
