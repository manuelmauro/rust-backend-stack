use cucumber::{then, when};
use reqwest::{self, StatusCode};
use rust_backend_stack::app::users::schema::{LoginUser, NewUser, User, UserBody};

use crate::World;

#[when(expr = "we register with username {word}, email {word}, and password {word}")]
async fn register(w: &mut World, username: String, email: String, password: String) {
    let client = reqwest::Client::new();
    let resp = client
        .post("http://localhost:8080/api/users")
        .json(&UserBody {
            user: NewUser {
                username: username,
                email: email,
                password: password,
            },
        })
        .send()
        .await
        .expect("The request should succed");

    w.status = Some(resp.status());
}

#[then("either the user exists or a new user is created")]
async fn user_existing_or_registered(w: &mut World) {
    if let Some(status) = &w.status {
        assert!(
            *status == StatusCode::OK || *status == StatusCode::UNPROCESSABLE_ENTITY
        );
    }
}

#[when(expr = "we login with email {word} and password {word}")]
async fn login(w: &mut World, email: String, password: String) {
    let client = reqwest::Client::new();
    let user = client
        .post("http://localhost:8080/api/users/login")
        .json(&UserBody {
            user: LoginUser {
                email: email,
                password: password,
            },
        })
        .send()
        .await
        .expect("The request should succed")
        .json::<UserBody<User>>()
        .await
        .expect("The body is properly formatted");

    w.email = Some(user.user.email);
    w.username = Some(user.user.username);
    w.token = Some(user.user.token);
}

#[then("we receive an authorization token")]
async fn authorization_token(w: &mut World) {
    assert!(w.email.is_some());
    assert!(w.username.is_some());
    assert!(w.token.is_some());
}
