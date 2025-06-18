use super::util::AppError;
use crate::lti_app_models::dyn_reg::get_dynamic_registration;
use crate::lti_models::dynamic_registration::DynRegLtiMessage;
use crate::lti_models::dynamic_registration::DynamicRegistrationResponse;
use crate::lti_models::platform_openid_configuration::PlatformOpenidConfiguration;
use crate::lti_models::platform_openid_configuration::PlatformOpenidLtiConfiguration;
use askama_axum::Template;
use axum::{extract::State, Form};
use color_eyre::eyre::eyre;
use entity::{registration, registration::Entity as Registration};
use sea_orm::{DatabaseConnection, EntityTrait, Set};
use serde::Deserialize;
use std::sync::Arc;
use url::Url;

#[derive(Deserialize, Debug)]
pub struct DynRegInitiateQueryParams {
    openid_configuration: String,
    registration_token: Option<String>,
}

#[derive(Template)]
#[template(path = "dyn_reg_initiate.html")]
pub struct DynRegInitiateTemplate {
    oauth_configuration_str: String,
    registration_resp_str: String,
    client_id: String,
}

// TODO simplify this function...
pub async fn dyn_reg_initiate(
    State(db): State<Arc<DatabaseConnection>>,
    Form(query_params): Form<DynRegInitiateQueryParams>,
) -> Result<DynRegInitiateTemplate, AppError> {
    // TODO: probably better error handling istead of just using eyre
    let oidc_conf_url = &query_params.openid_configuration;
    let oidc_conf_resp = reqwest::get(oidc_conf_url).await?;
    // return with error if resp is non-2xx
    let openid_configuration: PlatformOpenidConfiguration =
        oidc_conf_resp.error_for_status()?.json().await?;
    ensure_oidc_issuer_and_conf_url_match(oidc_conf_url, &openid_configuration.issuer)?;

    // For later use in the template (before we consume the oauth config)
    let oauth_configuration_str = serde_json::to_string_pretty(&openid_configuration)?;

    let PlatformOpenidConfiguration {
        registration_endpoint,
        scopes_supported,
        issuer,
        authorization_endpoint,
        jwks_uri,
        lti_configuration:
            PlatformOpenidLtiConfiguration {
                messages_supported, ..
            },
        ..
    } = openid_configuration;

    let messages: Vec<DynRegLtiMessage> =
        messages_supported.into_iter().map(|m| m.into()).collect();

    let reg_uuid = uuid::Uuid::new_v4().to_string();

    let dr = get_dynamic_registration(&reg_uuid, messages, scopes_supported);
    let client = reqwest::Client::new();
    let mut reg_request = client.post(&registration_endpoint);
    if let Some(token) = &query_params.registration_token {
        reg_request = reg_request.header("Authorization", format!("Bearer {}", token));
    }
    let reg_resp = reg_request.json(&dr).send().await?;
    let reg_result: DynamicRegistrationResponse = reg_resp.error_for_status()?.json().await?;

    let registration_resp_str = serde_json::to_string_pretty(&reg_result)?;

    let new_reg = registration::ActiveModel {
        uuid: Set(reg_uuid),
        issuer: Set(issuer),
        platform_auth_endpoint: Set(authorization_endpoint),
        platform_jwks_uri: Set(jwks_uri),
        client_id: Set(reg_result.client_id.clone()),
        ..Default::default()
    };
    Registration::insert(new_reg).exec(db.as_ref()).await?;

    let res = DynRegInitiateTemplate {
        client_id: reg_result.client_id,
        oauth_configuration_str,
        registration_resp_str,
    };
    Ok(res)
}

fn ensure_oidc_issuer_and_conf_url_match(conf: &str, issuer: &str) -> Result<(), AppError> {
    let conf = Url::parse(conf)?;
    let issuer = Url::parse(issuer)?;
    let matches = conf.host() == issuer.host()
        && conf.port_or_known_default() == issuer.port_or_known_default()
        && conf.scheme() == issuer.scheme();

    // TODO: remove this special case when canvas local is fixed
    let matches = matches || issuer.host_str() == Some("canvas.instructure.com");

    if matches {
        Ok(())
    } else {
        Err(eyre!("Issuer and configuration URL do not match").into())
    }
}
