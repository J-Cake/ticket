use actix_web::web;
use jsonwebtoken::jwk::JwkSet;
use crate::config::Config;

pub async fn get(http: web::Data<reqwest::Client>, config: web::Data<Config>) -> std::io::Result<JwkSet> {
    let jwk = http
        .get(&config.auth.jwks_url)
        .send()
        .await
        .expect("Failed to fetch JWKS");

    if !jwk.status().is_success() {
        log::error!("Failed to fetch JWKS: {}", jwk.status());
        return Err(std::io::Error::other("Failed to fetch JWKS"));
    }

    let max_age_secs = jwk
        .headers()
        .get("cache-control")
        .and_then(|v| v.to_str().ok())
        .and_then(|cc| {
            cc.split(',').find_map(|part| if let Some(stripped) = part.trim().strip_prefix("max-age=") {
                stripped.parse::<u64>().ok()
            } else {
                None
            })
        })
        .map(|i| i.min(config.auth.max_certificate_age))
        .unwrap_or(config.auth.max_certificate_age);

    let jwk: JwkSet = jwk.json().await.expect("JWK is not valid JSON");

    let jwk_serialised = serde_json::to_string_pretty(&jwk).expect("Failed to serialise JWK");
    log::info!("JWK cached");

    Ok(jwk)
}