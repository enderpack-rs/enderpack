use rust_dp::prelude::*;

fn main() {
    let selector = selector::all().distance(1.0..2.0);
    println!("{}", effect().give(selector, resource::effect::Speed));
}
