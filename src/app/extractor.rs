use crate::app::{error::Error, ApiContext};
use async_trait::async_trait;
use axum::extract::{FromRequestParts, State};
use axum::http::header::AUTHORIZATION;
use axum::http::{request::Parts, HeaderValue};
use hmac::{Hmac, Mac};
use jwt::{SignWithKey, VerifyWithKey};
use sha2::Sha384;
use time::OffsetDateTime;
use uuid::Uuid;

const DEFAULT_SESSION_LENGTH: time::Duration = time::Duration::weeks(2);
const SCHEME_PREFIX: &str = "Bearer ";

// Create alias for HMAC-SHA384
type HmacSha384 = Hmac<Sha384>;

/// Add this as a parameter to a handler function to require the user to be logged in.
///
/// Parses a JWT from the `Authorization: Token <token>` header.
pub struct AuthUser {
    pub user_id: Uuid,
}

#[derive(serde::Serialize, serde::Deserialize)]
struct AuthUserClaims {
    user_id: Uuid,
    /// Standard JWT `exp` claim.
    exp: i64,
}

impl AuthUser {
    pub(in crate::app) fn to_jwt(&self, ctx: &ApiContext) -> String {
        let hmac = HmacSha384::new_from_slice(ctx.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        AuthUserClaims {
            user_id: self.user_id,
            exp: (OffsetDateTime::now_utc() + DEFAULT_SESSION_LENGTH).unix_timestamp(),
        }
        .sign_with_key(&hmac)
        .expect("HMAC signing should be infallible")
    }

    /// Attempt to parse `Self` from an `Authorization` header.
    fn from_authorization(ctx: &ApiContext, auth_header: &HeaderValue) -> Result<Self, Error> {
        let auth_header = auth_header.to_str().map_err(|_| {
            log::debug!("Authorization header is not UTF-8");
            Error::Unauthorized
        })?;

        if !auth_header.starts_with(SCHEME_PREFIX) {
            log::debug!(
                "Authorization header is using the wrong scheme: {:?}",
                auth_header
            );
            return Err(Error::Unauthorized);
        }

        let token = &auth_header[SCHEME_PREFIX.len()..];

        let jwt =
            jwt::Token::<jwt::Header, AuthUserClaims, _>::parse_unverified(token).map_err(|e| {
                log::debug!(
                    "failed to parse Authorization header {:?}: {}",
                    auth_header,
                    e
                );
                Error::Unauthorized
            })?;

        let hmac = HmacSha384::new_from_slice(ctx.config.hmac_key.as_bytes())
            .expect("HMAC-SHA-384 can accept any key length");

        let jwt = jwt.verify_with_key(&hmac).map_err(|e| {
            log::debug!("JWT failed to verify: {}", e);
            Error::Unauthorized
        })?;

        let (_header, claims) = jwt.into();

        if claims.exp < OffsetDateTime::now_utc().unix_timestamp() {
            log::debug!("token expired");
            return Err(Error::Unauthorized);
        }

        Ok(Self {
            user_id: claims.user_id,
        })
    }
}

#[async_trait]
impl FromRequestParts<ApiContext> for AuthUser
where
    ApiContext: Send + Sync,
{
    type Rejection = Error;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &ApiContext,
    ) -> Result<Self, Self::Rejection> {
        let ctx: State<ApiContext> = State::from_request_parts(parts, state)
            .await
            .expect("BUG: ApiContext was not added as an extension");

        // Get the value of the `Authorization` header, if it was sent at all.
        let auth_header = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(Error::Unauthorized)?;

        Self::from_authorization(&ctx, auth_header)
    }
}
