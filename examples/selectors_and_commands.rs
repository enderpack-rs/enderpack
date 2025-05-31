use enderpack::prelude::*;
use serde_json::json;

fn main() {
    let player_selector = PlayerSelector::new("enderprism");
    println!(
        "{}",
        effect()
            .give(player_selector, resource::effect::Speed)
            .duration(4)
    );
    let target_selector = all().distance(..10.0);
    println!(
        "{}",
        tellraw(
            target_selector,
            json!(["Some ",{"strikethrough":true,"text":"crossed"}," text"])
        )
    );
}
