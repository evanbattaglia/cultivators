// These should really all be &' static, but I'd either have to generate them either when I
// don't need them (e.g. some CLI commands) or wrap in once_cell::sync::Lazy + Mutex

pub const INSTALL_URI: &str = "/install";
pub const LOGIN_BASE_URI: &str = "/login";
pub const LAUNCH_URI: &str = "/launch";
pub const REAL_LAUNCH_URI: &str = "/real_launch";
pub const JWKS_URI: &str = "/jwks";

use crate::env::BASE_URL_CACHED;

pub fn dyn_reg_initiation_uri() -> String {
    format!("{}{}", &*BASE_URL_CACHED, INSTALL_URI)
}

pub fn login_path(registration_id: &str) -> String {
    format!("{}/{}", &*LOGIN_BASE_URI, registration_id)
}

pub fn login_uri(registration_id: &str) -> String {
    format!("{}{}", &*BASE_URL_CACHED, login_path(registration_id))
}

pub fn launch_uri() -> String {
    format!("{}{}", &*BASE_URL_CACHED, LAUNCH_URI)
}

pub fn real_launch_uri() -> String {
    format!("{}{}", &*BASE_URL_CACHED, REAL_LAUNCH_URI)
}

pub fn jwks_uri() -> String {
    if let Ok(jwks_uri) = std::env::var("CULTIVATORS_JWKS_URI") {
        return jwks_uri;
    }
    format!("{}{}", &*BASE_URL_CACHED, JWKS_URI)
}
