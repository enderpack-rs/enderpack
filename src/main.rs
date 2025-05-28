use rust_dp::data_types::selector::*;

fn main() {
    let selector = all().distance(1.0..5.0);
    println!("{}", selector);
}
