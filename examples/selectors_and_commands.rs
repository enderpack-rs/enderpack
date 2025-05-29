use enderpack::prelude::*;

fn main() {
    let player_selector = PlayerSelector::new("enderprism");
    println!(
        "{}",
        effect()
            .give(player_selector, resource::effect::Speed)
            .duration(4)
    );
    let selector = TargetSelector::all().distance(..10.0);
}
