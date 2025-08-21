use actix_web::{
    web,
};
use redis::{
    aio::MultiplexedConnection,
};
use serde::{
    Deserialize,
    Serialize
};

pub type RedisHandle = web::Data<tokio::sync::Mutex<MultiplexedConnection>>;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TokenClaims {
    pub sub: String,
    pub preferred_username: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Token {
    pub claims: TokenClaims,
    pub username: Username
}

impl AsRef<Token> for &Token {
    fn as_ref(&self) -> &Token {
        self
    }
}

pub type Username = String;