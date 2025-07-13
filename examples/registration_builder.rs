use enderpack::prelude::*;
use serde_json::json;

fn main() {
    let dp = Datapack::new(get_namespace!(), "My awesome datapack!", Version::new(71))
        .add_function(Tag::Load, load);
    println!("{:#?}", dp);
}

#[func]
fn load() {
    // tellraw(all(), json!("hi!"));
    // effect().give(all(), resource::Effect::Speed);
    //
    // let test_scoreboard = resource::Objective::new("test").unwrap();
    // scoreboard()
    //     .objectives()
    //     .add(&test_scoreboard, resource::Criteria::Dummy);
    //
    // scoreboard().players().display().number_format(
    //     all().distance(..5.0).sort(Sort::Nearest).limit(10),
    //     &test_scoreboard,
    //     resource::NumberFormat::Styled(json!({"bold": true})),
    // );
    // scoreboard()
    //     .objectives()
    //     .set_display(resource::ScoreboardSlot::SidebarTeam(resource::Color::Red))
    //     .objective(&test_scoreboard);
    // scoreboard().objectives().modify(
    //     &test_scoreboard,
    //     objectives::modify::Mode::DisplayAutoupdate(true),
    // );
    //
    // let a1: Score = 0;
    // let a2: Score = 9 + 10;
    let a: Score = -10;
    let b: Score = a;
    let c: Score = &b + 12 + 6 + 15;
    // This is more optimised as the number is resolved at compile-time.
    let d: Score = &b + (12 + 6 + 15);
    let e: Score = 32 + c;
}

// `registration_builder` is an example, and Rust treats each example
// as a separate crate. We are at the root of this crate.
