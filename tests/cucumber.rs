use cucumber::World as _;
use reqwest::Response;

mod step_defs;

#[derive(cucumber::World, Debug, Default)]
struct World {
    response: Option<Response>,
}

#[tokio::main]
async fn main() {
    World::cucumber()
        .max_concurrent_scenarios(1)
        .run("tests/features/user.feature")
        .await;
}
