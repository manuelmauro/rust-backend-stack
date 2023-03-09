use cucumber::{when, World as _};

use rust_backend_stack::app::users::{NewUser, UserBody};

#[derive(cucumber::World, Debug, Default)]
struct World {}

#[when(expr = "a user registers as {word} with password {word}")]
async fn register_user(_w: &mut World, username: String, password: String) {
    let client = reqwest::Client::new();
    let res = client
        .post("http://localhost:8080/api/users")
        .json(&UserBody {
            user: NewUser {
                username: username.clone(),
                email: format!("{}@example.com", username),
                password: password,
            },
        })
        .send()
        .await
        .expect("Can't reach the server");

    assert!(res.status() == 200)
}

#[tokio::main]
async fn main() {
    World::run("tests/user.feature").await;
}
