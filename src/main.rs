use rust_dp::commands::effect::*;
use rust_dp::data_types::resource::effect::EffectResource;
use rust_dp::data_types::selector::*;

fn main() {
    let selector = all().distance(1.0..5.0).build().unwrap();
    println!(
        "{}",
        effect()
            .give(selector, EffectResource::Speed)
            .build()
            .unwrap()
    );
}
