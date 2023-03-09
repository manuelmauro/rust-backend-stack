use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

#[derive(sqlx::Type, Serialize, Deserialize)]
pub struct Timestamptz(#[serde(with = "time::serde::rfc3339")] pub OffsetDateTime);
