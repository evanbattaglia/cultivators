use super::util::AppError;
use crate::lti_app_models::launch_integrity_validator::LaunchIntegrityValidator;
use askama_axum::Template;
use axum::extract::State;
use axum::{http::HeaderMap, Json};
use color_eyre::eyre::eyre;
use entity::{nonce, nonce::Entity as Nonce};
use entity::{registration, registration::Entity as Registration};
use jsonwebtoken::{jwk::JwkSet, Algorithm, DecodingKey, TokenData, Validation};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::{DatabaseConnection, ModelTrait, QueryFilter};
use serde::Deserialize;
use std::sync::Arc;

#[derive(Deserialize)]
pub struct RealLaunchPayload {
    state: String,
    id_token: String,
    origin: String,
}

fn domains_match(url1: &str, url2: &str) -> Result<bool, AppError> {
    let url1 = url::Url::parse(url1)?;
    let url2 = url::Url::parse(url2)?;
    let d1 = url1.domain().ok_or_else(|| eyre!("No domain in URL"))?;
    let d2 = url2.domain().ok_or_else(|| eyre!("No domain in URL"))?;
    Ok(d1 == d2)
}

#[derive(Template)]
#[template(path = "real_launch.html")]
pub(super) struct RealLaunchTemplate {
    state: String,
    id_token_jwt: TokenData<serde_json::Value>,
}

pub async fn real_launch(
    header_map: HeaderMap,
    State(db): State<Arc<DatabaseConnection>>,
    Json(real_launch_payload): Json<RealLaunchPayload>,
) -> Result<RealLaunchTemplate, AppError> {
    let RealLaunchPayload {
        state,
        id_token,
        origin,
    } = real_launch_payload;
    let liv = header_map
        .get("X-Cultivators-Launch-Integrity-Validator")
        .ok_or_else(|| eyre!("No LTI verification key"))?
        .to_str()
        .map_err(|_| eyre!("Could not convert LTI verification key to string"))?;
    let LaunchIntegrityValidator {
        nonce,
        reg_uuid,
        exp: _,
    } = LaunchIntegrityValidator::from_jwt(liv)?;

    let nonce_model = Nonce::find()
        .filter(nonce::Column::Uuid.eq(&nonce))
        .one(db.as_ref())
        .await?;
    let nonce_model = nonce_model.ok_or_else(|| eyre!("Nonce not found"))?;
    nonce_model.delete(db.as_ref()).await?;

    let reg = Registration::find()
        .filter(registration::Column::Uuid.eq(&reg_uuid))
        .one(db.as_ref())
        .await?;
    let reg = reg.ok_or_else(|| eyre!("Registration not found"))?;
    if !domains_match(&reg.platform_auth_endpoint, &origin)? {
        Err(eyre!("Origin does not match registration"))?;
    }

    let mut jwk_errs: Vec<jsonwebtoken::errors::Error> = vec![];

    let platform_jwks: JwkSet = reqwest::get(&reg.platform_jwks_uri).await?.json().await?;
    for jwk in platform_jwks.keys {
        let decoding_key = DecodingKey::from_jwk(&jwk)?;
        dbg!(&id_token);
        let mut validation = Validation::new(Algorithm::RS256);
        validation.set_audience(&[&reg.client_id]);

        let id_token_jwt_result =
            jsonwebtoken::decode::<serde_json::Value>(&id_token, &decoding_key, &validation);

        match id_token_jwt_result {
            Ok(id_token_jwt) => {
                return Ok(RealLaunchTemplate {
                    state,
                    id_token_jwt,
                })
            }
            Err(err) => {
                jwk_errs.push(err);
            }
        }
    }

    Err(eyre!(
        "Could not validate launch token with any available jwk. Errors for each jwk: {:?}",
        jwk_errs
    ))?
}
