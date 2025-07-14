use enderpack::prelude::*;
use serde_json::json;

fn main() {
    let dp = Datapack::new(get_namespace!(), "My awesome datapack!", Version::new(71))
        .add_function(Tag::Load, load);
    println!("{:#?}", dp);
}

#[func]
fn load() {
    let a: Score = -10;
    let b: Score = a;
    let c: Score = &b + 12 + 6 + 15;
    // This is more optimised as the number is resolved at compile-time.
    let d: Score = &b + (12 + 6 + 15);
    let e: Score = 32 + c;
    hello_world();
}

#[func]
fn hello_world() {
    tellraw(all(), json!("hello world"));
}

// `registration_builder` is an example, and Rust treats each example
// as a separate crate. We are at the root of this crate.
