use chrono::Utc;
use jsonwebtoken::{DecodingKey, TokenData, Validation};
use jsonwebtoken::{EncodingKey, Header};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

pub mod filter;

// TODO: Proper error handling

static ONE_WEEK: i64 = 60 * 60 * 24 * 7; // in seconds

#[derive(Deserialize, Serialize)]
pub struct Claims {
    // issued at
    pub iat: i64,
    // expiration
    pub exp: i64,
    // data:
    pub id: String,
    pub user: String,
    pub role: String,
    pub session: String,
}

pub struct CurrentUser {
    pub id: String,
    pub username: String,
    pub role: String,
}

pub fn generate_token(
    id: &str,
    user: &str,
    role: &str,
    session: &str,
    secret: &str,
) -> String {
    let now = Utc::now().timestamp_nanos() / 1_000_000_000; // nanosecond -> second
    let payload = Claims {
        iat: now,
        exp: now + ONE_WEEK,
        id: id.to_string(),
        user: user.to_string(),
        role: role.to_string(),
        session: session.to_string(),
    };

    jsonwebtoken::encode(
        &Header::default(),
        &payload,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .unwrap()
}

// FIXME: This needs to gracefully send back an error if the session doesn't exist
pub fn generate_session(
    con: &mut redis::Connection,
    user_id: &str,
) -> String {
    let session = Uuid::new_v4().to_simple().to_string();
    set_session(con, user_id, &session);

    session
}

pub fn decode_token(
    token: &str,
    secret: &str,
) -> jsonwebtoken::errors::Result<TokenData<Claims>> {
    jsonwebtoken::decode::<Claims>(
        &token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
}

pub fn set_session(
    con: &mut redis::Connection,
    user_id: &str,
    session: &str,
) {
    let _: () = redis::cmd("SET")
        .arg(user_id)
        .arg(session)
        .query(con)
        .expect("Failed to store the user session in redis");
}

pub fn get_session(
    con: &mut redis::Connection,
    user_id: &str
) -> String {
    let session: String = redis::cmd("GET").arg(user_id).query(con)
        .expect("Failed to retrieve the user session from redis");

    session
}

pub fn remove_session(
    con: &mut redis::Connection,
    user_id: &str
) {
    let _: () = redis::cmd("DEL")
        .arg(user_id)
        .query(con)
        .expect("Failed to delete user session in redis");
}

pub fn verify_token(
    con: &mut redis::Connection,
    token_data: &TokenData<Claims>,
) -> Result<(), String> {
    let stored_session = get_session(con, &token_data.claims.id);

    if stored_session == token_data.claims.session {
        Ok(())
    } else {
        Err("Invalid token".to_string())
    }
}

pub fn get_current_user(
    con: &mut redis::Connection,
    token_data: &TokenData<Claims>,
) -> Option<CurrentUser> {
    verify_token(con, &token_data).expect("Can't verify token");

    Some(CurrentUser{
        id: token_data.claims.id.clone(),
        username: token_data.claims.user.clone(),
        role: token_data.claims.role.clone(),
    })
}