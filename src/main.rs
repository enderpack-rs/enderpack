use rust_dp::commands::effect::*;
use rust_dp::data_types::resource::effect::EffectResource::*;
use rust_dp::data_types::selector::*;

fn main() {
    let selector = all().distance(1.0..2.0);
    println!("{}", effect().give(selector, Speed));
}
