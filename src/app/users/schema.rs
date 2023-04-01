use utoipa::ToSchema;

/// A wrapper type for all requests/responses from these routes.
#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
#[aliases(
    NewUserBody = UserBody<NewUser>,
    LoginUserBody = UserBody<LoginUser>,
    UpdateUserBody = UserBody<UpdateUser>,
    UserResponse = UserBody<User>
)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
pub struct NewUser {
    pub username: String,
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, serde::Serialize, ToSchema)]
pub struct LoginUser {
    pub email: String,
    pub password: String,
}

#[derive(serde::Deserialize, Default, PartialEq, Eq, ToSchema)]
#[serde(default)]
pub struct UpdateUser {
    pub email: Option<String>,
    pub username: Option<String>,
    pub password: Option<String>,
}

#[derive(serde::Serialize, serde::Deserialize, ToSchema)]
pub struct User {
    pub email: String,
    pub token: String,
    pub username: String,
}
