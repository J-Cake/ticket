use std::{
    ops::Deref
};
use actix_web::{
    HttpMessage,
    web,
    middleware::Next,
    dev::ServiceResponse,
    dev::ServiceRequest,
    body::MessageBody,
    body::EitherBody,
    body::BoxBody,
    HttpResponse
};
use jsonwebtoken::{
    jwk::JwkSet,
    DecodingKey,
    Algorithm,
    jwk::PublicKeyUse
};
use redis::{
    AsyncCommands,
    SetExpiry,
    SetOptions
};
use crate::{
    oidc::RedisHandle,
    oidc::Token,
    oidc::TokenClaims,
    config::Config
};

pub(crate) async fn authenticate_middleware<B>(req: ServiceRequest, next: Next<B>) -> Result<ServiceResponse<EitherBody<B, BoxBody>>, actix_web::Error>
where
    B: MessageBody + 'static,
{
    let Some(header) = req.headers().get("Authorization") else {
        return Ok(req
            .into_response(HttpResponse::Unauthorized().finish())
            .map_into_right_body());
    };

    let Some(redis) = req.app_data::<RedisHandle>().cloned() else {
        log::error!("Redis client not found in middleware");
        return Ok(req
            .into_response(HttpResponse::InternalServerError().finish())
            .map_into_right_body());
    };

    let token = match header.to_str().map(ToOwned::to_owned) {
        Ok(token) if token.starts_with("Bearer ") => token[7..].to_owned(),
        Ok(_) => {
            return Ok(req
                .into_response(HttpResponse::Unauthorized().finish())
                .map_into_right_body());
        }
        Err(err) => {
            log::error!("Failed to parse Authorization header: {err}");
            return Ok(req
                .into_response(HttpResponse::Unauthorized().finish())
                .map_into_right_body());
        }
    };

    let Some(config) = req.app_data::<web::Data<Config>>() else {
        log::error!("Could not acquire configuration. Something's probably gone pretty badly wrong.");
        return Ok(req
            .into_response(HttpResponse::InternalServerError().finish())
            .map_into_right_body());
    };

    let mut redis = redis.lock().await;
    let jwt = match redis.get::<_, String>(format!("jwt:{token}")).await {
        Ok(jwt) => {
            log::debug!("JWT was cached - reusing");
            serde_json::from_str::<Token>(&jwt)?
        }
        Err(_) => {
            let Some(jwks) = req.app_data::<web::Data<JwkSet>>() else {
                log::error!("Jwks are not configured properly. Please check your configuration.");
                return Ok(req
                    .into_response(HttpResponse::InternalServerError().finish())
                    .map_into_right_body());
            };

            let mut validation = jsonwebtoken::Validation::new(Algorithm::RS256);

            validation.set_audience(&[&config.auth.audience]);
            validation.set_issuer(&[&config.auth.issuer]);

            let Some(jwt) = jwks
                .keys
                .iter()
                .filter(|i| {
                    i.common
                        .public_key_use
                        .as_ref()
                        .is_some_and(|key| *key == PublicKeyUse::Signature)
                })
                .filter_map(|key| DecodingKey::from_jwk(key).ok())
                .map(|key| jsonwebtoken::decode::<TokenClaims>(&token, &key, &validation))
                .find_map(Result::ok)
            else {
                log::warn!("Invalid token received.");
                return Ok(req
                    .into_response(HttpResponse::Unauthorized().finish())
                    .map_into_right_body());
            };

            Token {
                username: jwt.claims.preferred_username.clone(),
                claims: jwt.claims,
            }
        }
    };

    if let Ok(json_token) = serde_json::to_string_pretty(&jwt) {
        if let Err(err) = redis
            .set_options::<_, _, Option<String>>(
                format!("jwt:{token}"),
                json_token,
                SetOptions::default().with_expiration(SetExpiry::EX(config.auth.token_ttl)),
            )
            .await
        {
            log::error!("Failed to cache JWT: {err}");
            return Ok(req
                .into_response(HttpResponse::InternalServerError().finish())
                .map_into_right_body());
        };
    } else {
        return Ok(req
            .into_response(HttpResponse::InternalServerError().finish())
            .map_into_right_body());
    }

    req.extensions_mut().insert(jwt);
    Ok(next.call(req).await?.map_into_left_body())
}