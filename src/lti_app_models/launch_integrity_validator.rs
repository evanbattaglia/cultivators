//! A JWT used to secure the LTI 1.3 launch, created in the login request and validated in the
//! launch, used to ensure the same user (user agent) finishes the launch as started the launch.
//! This sort of validating token was traditionally stored in a cookie, but some browsers have
//! issues with cookies in iframes, especially when not using HTTPS (HTTP is used in local dev), so
//! in Cultivators, we use the platform's LTI Platform Storage to store it

use crate::crypto::files::private_key_pem;
use crate::crypto::files::public_key_pem;
use jsonwebtoken::{encode, Algorithm, Header};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LaunchIntegrityValidator {
    pub exp: i64,
    pub nonce: String,
    pub reg_uuid: String,
}

impl LaunchIntegrityValidator {
    pub fn to_jwt(&self) -> Result<String, jsonwebtoken::errors::Error> {
        encode(&Header::new(Algorithm::RS256), self, private_key_pem())
    }

    pub fn from_jwt(jwt: &str) -> Result<Self, jsonwebtoken::errors::Error> {
        jsonwebtoken::decode(
            jwt,
            public_key_pem(),
            &jsonwebtoken::Validation::new(Algorithm::RS256),
        )
        .map(|data| data.claims)
    }
}
