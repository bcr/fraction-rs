use cucumber_rust::{async_trait, Cucumber, World};
use std::convert::Infallible;

mod steps;

pub enum FractionWorld {
    Nothing,
    SomeString(String),
    ResultString(String),
}

#[async_trait(?Send)]
impl World for FractionWorld {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::Nothing)
    }
}

#[tokio::main]
async fn main() {
    Cucumber::<FractionWorld>::new()
        // Specifies where our feature files exist
        .features(&["./features"])
        // Adds the implementation of our steps to the runner
        .steps(steps::fractionsteps::steps())
        // Parses the command line arguments if passed
        .cli()
        // Runs the Cucumber tests and then exists
        .run_and_exit()
        .await
}
