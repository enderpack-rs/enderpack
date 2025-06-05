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
    scoreboard()
        .objectives()
        .add("test", resource::Criteria::Dummy);
    let a1: Score = 0;
    let a2: Score = -10;
    let a3: Score = 9 + 10;
    let b: Score = a2;
    // let c: Score = b + 12;
    // let d: Score = 32 + c;
}

// will expand to
// fn load() -> Function {
//     Function::new(stringify!(load))
//         .set_path(module_path!())
//         .add_command(tellraw(all(), json!("hi!")))
//         .add_command(effect().give(all(), resource::EffectResource::Speed))
// }

// `registration_builder` is an example, and Rust treats each example
// as a separate crate. We are at the root of this crate.
