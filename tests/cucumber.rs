use cucumber::World as _;
use reqwest::StatusCode;

mod step_defs;

#[derive(cucumber::World, Clone, Debug, Default)]
struct World {
    status: Option<StatusCode>,
    email: Option<String>,
    username: Option<String>,
    token: Option<String>,
}

#[tokio::main]
async fn main() {
    World::cucumber()
        .max_concurrent_scenarios(1)
        .run_and_exit("tests/features/user.feature")
        .await;
}
