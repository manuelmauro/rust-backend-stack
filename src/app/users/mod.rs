use self::schema::{LoginUser, NewUser, UpdateUser, User, UserBody};
use self::utils::{hash_password, verify_password};
use crate::app::error::{Error, ResultExt};
use crate::app::extractor::AuthUser;
use crate::app::{ApiContext, Result};
use axum::body::Body;
use axum::extract::State;
use axum::routing::{get, post};
use axum::{Json, Router};

/// All request/response schemas
pub mod schema;

/// Authentication utils
pub mod utils;

pub(crate) fn router() -> Router<ApiContext, Body> {
    Router::new()
        .route("/api/users", post(create_user))
        .route("/api/users/login", post(login_user))
        .route("/api/user", get(get_current_user).put(update_user))
}

#[utoipa::path(
    post,
    path = "/api/users",
    request_body = NewUserBody,
    responses(
        (status = 200, description = "Register a new user.", body = UserResponse)
    )
)]
async fn create_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<NewUser>>,
) -> Result<Json<UserBody<User>>> {
    let password_hash = hash_password(req.user.password).await?;

    let user_id = sqlx::query_scalar!(
        r#"insert into "user" (username, email, password_hash) values ($1, $2, $3) returning user_id"#,
        req.user.username,
        req.user.email,
        password_hash
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(UserBody {
        user: User {
            email: req.user.email,
            token: AuthUser { user_id }.to_jwt(&ctx),
            username: req.user.username,
        },
    }))
}

#[utoipa::path(
    post,
    path = "/api/users/login",
    request_body = LoginUserBody,
    responses(
        (status = 200, description = "Login with an existing user.", body = UserResponse)
    )
)]
async fn login_user(
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<LoginUser>>,
) -> Result<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"
            select user_id, email, username, password_hash 
            from "user" where email = $1
        "#,
        req.user.email,
    )
    .fetch_optional(&ctx.db)
    .await?
    .ok_or_else(|| Error::unprocessable_entity([("email", "does not exist")]))?;

    verify_password(req.user.password, user.password_hash).await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: AuthUser {
                user_id: user.user_id,
            }
            .to_jwt(&ctx),
            username: user.username,
        },
    }))
}

#[utoipa::path(
    get,
    path = "/api/user",
    security(
        ("api_key" = [])
    ),
    responses(
        (status = 200, description = "Get the currently logged-in user.", body = UserResponse)
    )
)]
async fn get_current_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
) -> Result<Json<UserBody<User>>> {
    let user = sqlx::query!(
        r#"select email, username from "user" where user_id = $1"#,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}

#[utoipa::path(
    put,
    path = "/api/user",
    request_body = UpdateUserBody,
    security(
        ("api_key" = [])
    ),
    responses(
        (status = 200, description = "Update information on the currently logged-in user.", body = UserResponse)
    )
)]
async fn update_user(
    auth_user: AuthUser,
    State(ctx): State<ApiContext>,
    Json(req): Json<UserBody<UpdateUser>>,
) -> Result<Json<UserBody<User>>> {
    if req.user == UpdateUser::default() {
        // If there's no fields to update, these two routes are effectively identical.
        return get_current_user(auth_user, State(ctx)).await;
    }

    let password_hash = if let Some(password) = req.user.password {
        Some(hash_password(password).await?)
    } else {
        None
    };

    let user = sqlx::query!(
        // This is how we do optional updates of fields without needing a separate query for each.
        // language=PostgreSQL
        r#"
            update "user"
            set email = coalesce($1, "user".email),
                username = coalesce($2, "user".username),
                password_hash = coalesce($3, "user".password_hash)
            where user_id = $4
            returning email, username
        "#,
        req.user.email,
        req.user.username,
        password_hash,
        auth_user.user_id
    )
    .fetch_one(&ctx.db)
    .await
    .on_constraint("user_username_key", |_| {
        Error::unprocessable_entity([("username", "username taken")])
    })
    .on_constraint("user_email_key", |_| {
        Error::unprocessable_entity([("email", "email taken")])
    })?;

    Ok(Json(UserBody {
        user: User {
            email: user.email,
            token: auth_user.to_jwt(&ctx),
            username: user.username,
        },
    }))
}
