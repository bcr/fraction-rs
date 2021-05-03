use cucumber_rust::{Steps};

// I don't know how to repent this sin. I just know it's a sin.
// Maybe just a venial sin.
#[path = "../../src/fraction.rs"]
mod fraction;

use crate::FractionWorld;

pub fn steps() -> Steps<crate::FractionWorld> {
    let mut steps: Steps<crate::FractionWorld> = Steps::new();

    steps.when_regex(
        r#"^we have the input "(.*)"$"#,
        |world, ctx| match world {
            // FractionWorld::Nothing => FractionWorld::ResultString(String::from("3_1/2")),
            FractionWorld::Nothing => FractionWorld::ResultString(String::from(fraction::process_input(&ctx.matches[1]).unwrap())),
            _ => panic!("Invalid world state")
        },
    )
    .then_regex(
        r#"^the output should be "(.*)"$"#,
        |world, ctx| {
            match world {
                FractionWorld::ResultString(x) => assert_eq!(x, ctx.matches[1]),
                _ => panic!("Invalid world state")
            }
    
            FractionWorld::Nothing
        }
    );

    steps
}
