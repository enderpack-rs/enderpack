use rust_dp::prelude::*;

fn main() {
    let selector = Selector::all().distance(..10.0);
    println!(
        "{}",
        effect().give(selector, resource::effect::Speed).duration(4)
    );
}
