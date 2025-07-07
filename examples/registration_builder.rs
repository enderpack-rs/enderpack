use enderpack::prelude::*;
use serde_json::json;

fn main() {
    let dp = Datapack::new(get_namespace!(), "My awesome datapack!", Version::new(71))
        .add_function(Tag::Load, load);
    println!("{:#?}", dp);
}

#[func]
fn load() {
    tellraw(all(), json!("hi!"));
    effect().give(all(), resource::Effect::Speed);
    let test_scoreboard = resource::Objective::new("test").unwrap();
    scoreboard()
        .objectives()
        .add(&test_scoreboard, &resource::Criteria::Dummy);
    scoreboard().players().display().number_format(
        all_players(),
        &test_scoreboard,
        resource::NumberFormat::Styled(json!({"bold": true})),
    );
    scoreboard()
        .objectives()
        .set_display(resource::ScoreboardSlot::SidebarTeam(resource::Color::Red))
        .objective(resource::Objective::new("test").unwrap());
    scoreboard().objectives().modify(
        &test_scoreboard,
        objectives::modify::Mode::DisplayAutoupdate(true),
    );
    let a1: Score = 0;
    let a2: Score = -10;
    let a3: Score = 9 + 10;
    let b: Score = a2;
    // let c: Score = b + 12; // Not implemented
    // let d: Score = 32 + c; // Not implemented
}

// `registration_builder` is an example, and Rust treats each example
// as a separate crate. We are at the root of this crate.
