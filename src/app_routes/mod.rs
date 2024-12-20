mod dyn_reg;
/// LTI Login
mod launch;
/// LTI Dynamic Registration
pub mod login; // TODO make not public, move shared code used by cli
mod real_launch;

mod util;

use crate::crypto;
use crate::{app_uris, seaorm_setup};
use askama_axum::Template;
use axum::routing::get;
use axum::Json;
use axum::{routing::post, Router};
use serde::Serialize;
use std::sync::Arc;

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate {
    dynamic_registration_uri: String,
    public_jwk_set_url: String,
}

pub async fn index_handler() -> IndexTemplate {
    IndexTemplate {
        dynamic_registration_uri: app_uris::dyn_reg_initiation_uri(),
        public_jwk_set_url: app_uris::jwks_uri(),
    }
}

pub async fn jwks() -> Json<impl Serialize> {
    Json(crypto::files::get_jwk_set_cached())
}

pub async fn app() -> Router {
    let db_conn = seaorm_setup::create_db_connection()
        .await
        .unwrap_or_else(|e| panic!("Couldn't connect to db: {e:?}"));

    Router::new()
        .route("/", get(index_handler))
        .route(
            &app_uris::login_path(":registration_uuid"),
            post(login::login),
        )
        .route(app_uris::LAUNCH_URI, post(launch::launch))
        .route(app_uris::REAL_LAUNCH_URI, post(real_launch::real_launch))
        .route(app_uris::INSTALL_URI, get(dyn_reg::dyn_reg_initiate))
        .route(app_uris::JWKS_URI, get(jwks))
        .with_state(Arc::new(db_conn))
}
