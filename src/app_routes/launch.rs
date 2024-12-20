use crate::app_uris::real_launch_uri;
use askama_axum::Template;
use axum::Form;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct LaunchPayload {
    state: String,
    lti_storage_target: String,
    id_token: String,
}

#[derive(Deserialize, Template)]
#[template(path = "launch.html")]
pub struct LaunchTemplate {
    state: String,
    id_token: String,
    lti_storage_target: String,
    real_launch_uri: String,
}

pub async fn launch(Form(launch_payload): Form<LaunchPayload>) -> LaunchTemplate {
    let LaunchPayload {
        state,
        lti_storage_target,
        id_token,
    } = launch_payload;
    LaunchTemplate {
        state,
        id_token,
        lti_storage_target,
        real_launch_uri: real_launch_uri(),
    }
}
