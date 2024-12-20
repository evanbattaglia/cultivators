use super::util::AppError;
use crate::app_uris;
use crate::lti_app_models::launch_integrity_validator::LaunchIntegrityValidator;
use crate::lti_models::login_request;
use askama_axum::Template;
use axum::extract::State;
use axum::{extract::Path, Form};
use color_eyre::eyre::eyre;
use entity::{nonce, nonce::Entity as Nonce};
use entity::{registration, registration::Entity as Registration};
use sea_orm::ColumnTrait;
use sea_orm::EntityTrait;
use sea_orm::{DatabaseConnection, NotSet, QueryFilter, Set};
use serde::Deserialize;
use serde::Serialize;
use std::sync::Arc;

// TODO probably move to some sort of shared code place since this is now used in the CLI

#[derive(Deserialize)]
pub struct LoginPayload {
    pub lti_message_hint: String,
    pub login_hint: String,
    pub client_id: String,
    pub lti_storage_target: String,
}

#[derive(Deserialize)]
pub struct LoginPath {
    pub registration_uuid: String,
}

#[derive(Serialize, Template)]
#[template(path = "login.html")]
pub struct LoginTemplate {
    platform_authorize_uri: String,
    launch_integrity_validator: String,
    state: String,
    lti_storage_target: String,
}

pub async fn _login(
    db: &DatabaseConnection,
    path: LoginPath,
    payload: LoginPayload,
) -> Result<LoginTemplate, color_eyre::eyre::Error> {
    let launch_uri = app_uris::launch_uri();
    let reg = Registration::find()
        .filter(registration::Column::Uuid.eq(&path.registration_uuid))
        .one(db)
        .await?
        .ok_or_else(|| eyre!("Registration not found".to_string()))?;

    if reg.client_id != payload.client_id {
        return Err(eyre!("Client ID does not match".to_string()));
    }

    let state = uuid::Uuid::new_v4().to_string();
    let nonce = uuid::Uuid::new_v4().to_string();
    let nonce_model = nonce::ActiveModel {
        id: NotSet,
        uuid: Set(nonce.clone()),
    };
    Nonce::insert(nonce_model).exec(db).await?;

    let deployment_params: login_request::DeploymentParams = login_request::DeploymentParams {
        redirect_uri: &launch_uri,
        client_id: &reg.client_id,
        platform_authorize_uri: &reg.platform_auth_endpoint,
    };

    let launch_params = login_request::LaunchParams {
        lti_message_hint: &payload.lti_message_hint,
        login_hint: &payload.login_hint,
        state: &state,
        nonce: &nonce,
    };

    let platform_authorize_uri = login_request::build_url(&deployment_params, &launch_params)?;

    let liv = LaunchIntegrityValidator {
        nonce,
        reg_uuid: reg.uuid,
        // TODO move into LaunchIntegrityValidator code
        exp: chrono::Utc::now().timestamp() + 60 * 60,
    }
    .to_jwt()?;

    Ok(LoginTemplate {
        platform_authorize_uri,
        lti_storage_target: payload.lti_storage_target,
        launch_integrity_validator: liv,
        state,
    })
}

pub async fn login(
    State(db): State<Arc<DatabaseConnection>>,
    Path(path): Path<LoginPath>,
    Form(payload): Form<LoginPayload>,
) -> Result<LoginTemplate, AppError> {
    Ok(_login(db.as_ref(), path, payload).await?)
}
