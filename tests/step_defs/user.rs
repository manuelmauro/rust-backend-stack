use cucumber::{then, when};
use reqwest::{self, StatusCode};
use rust_backend_stack::app::users::{NewUser, UserBody};

use crate::World;

#[when(expr = "we register with username {word} and password {word}")]
async fn register(w: &mut World, username: String, password: String) {
    let client = reqwest::Client::new();
    let resp = client
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
        .expect("The request should succed");

    w.response = Some(resp);
}

#[then("either the user exists or a new user is created")]
async fn user_existing_or_registered(w: &mut World) {
    if let Some(resp) = &w.response {
        assert!(
            resp.status() == StatusCode::OK || resp.status() == StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}
