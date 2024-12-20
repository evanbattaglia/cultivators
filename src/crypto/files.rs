//! Higher-order functions for working with files
//! or the defaults used for the tool

use crate::crypto::{
    jwkgen::{
        decoding_key_from_private_pem, encoding_key_from_private_pem, jwk_set_from_private_key_pem,
    },
    keygen::{generate_rsa_private_key, KeygenError},
};
use crate::env::bool_from_env;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;
use once_cell::sync::Lazy;
use once_cell::sync::OnceCell;
use tracing::{error, info, warn};
use zeroize::Zeroizing;

use super::jwkgen::JwkGenError;
use std::path::PathBuf;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwkError {
    #[error("Failed to read private key PEM file referenced by CULTIVATORS_PRIVATE_KEY_PEM_FILE")]
    PrivateKeyReadError(#[from] std::io::Error),
    #[error("Failed to generate private key PEM")]
    PrivateKeyGenError(#[from] KeygenError),
    #[error("Failed to generate JWK from private key PEM")]
    JwkGenerationError(#[from] JwkGenError),
}

fn gen_private_key_and_save_to_file(filename: &str) -> Result<Zeroizing<String>, JwkError> {
    let priv_pem_str = generate_rsa_private_key()?;
    std::fs::write(filename, priv_pem_str.as_bytes())?;
    Ok(priv_pem_str)
}

/// Loads prvate key PEM from one of these sources:
/// 1. PathBuf to private key PEM file, or
/// 2. PEM in CULTIVATORS_PRIVATE_KEY_PEM_FILE environment variable, or
/// 2. "private.pem" in current directory
/// 3. "default.private.pem" compiled into the binary
pub fn load_private_key_pem_str(
    private_key_pem: Option<&PathBuf>,
) -> Result<Zeroizing<String>, JwkError> {
    static WARNING: OnceCell<()> = OnceCell::new();
    if let Some(path_buf) = private_key_pem {
        return Ok(Zeroizing::new(std::fs::read_to_string(path_buf)?));
    }
    let pem_filename = std::env::var("CULTIVATORS_PRIVATE_KEY_PEM_FILE")
        .expect("CULTIVATORS_PRIVATE_KEY_PEM_FILE not set");

    let res = std::fs::read_to_string(&pem_filename);
    match res {
        Ok(pem_str) => {
            WARNING.get_or_init(|| {
                info!("Using existing private key PEM file at {}", pem_filename);
            });
            Ok(Zeroizing::new(pem_str))
        }
        Err(e) => {
            if bool_from_env("CULTIVATORS_PRIVATE_KEY_PEM_FILE_CREATE_IF_MISSING") {
                WARNING.get_or_init(|| {
                    warn!(
                        "Generating private key PEM file and saving in {}",
                        pem_filename
                    );
                });
                gen_private_key_and_save_to_file(&pem_filename)
            } else {
                error!("Failed to read private key PEM file at {}: {}; CULTIVATORS_PRIVATE_KEY_PEM_FILE_CREATE_IF_MISSING is not set so didn't try to create", pem_filename, e);
                Err(e.into())
            }
        }
    }
}

static PRIVATE_KEY_CACHED: Lazy<EncodingKey> = Lazy::new(|| {
    let pem = load_private_key_pem_str(None).unwrap();
    encoding_key_from_private_pem(&pem).unwrap()
});

static PUBLIC_KEY_CACHED: Lazy<DecodingKey> = Lazy::new(|| {
    let pem = load_private_key_pem_str(None).unwrap();
    decoding_key_from_private_pem(&pem).unwrap()
});

static PUBLIC_JWK_SET_CACHED: Lazy<JwkSet> = Lazy::new(|| get_jwk_set(None).unwrap());

/// Ensures the private key pem is loaded. Should be run at beginning of app.
pub fn ensure_tool_keys_loaded() {
    Lazy::force(&PRIVATE_KEY_CACHED);
    Lazy::force(&PUBLIC_KEY_CACHED);
    Lazy::force(&PUBLIC_JWK_SET_CACHED);
}

/// Get the private key pem. Should be used in the app whenever the key is needed. Using this will
/// ensure we don't forget to initialize at the beginning of the app.
pub fn private_key_pem() -> &'static EncodingKey {
    Lazy::get(&PRIVATE_KEY_CACHED).unwrap()
}

pub fn public_key_pem() -> &'static DecodingKey {
    Lazy::get(&PUBLIC_KEY_CACHED).unwrap()
}

pub fn get_jwk_set_cached() -> &'static JwkSet {
    Lazy::get(&PUBLIC_JWK_SET_CACHED).unwrap()
}

/// Returns a Jwk which can be serialized with
/// serde_json and used in the LTI tool.
/// Arguments are the same as load_private_key_pem_str, plus a key_id ("cultivators" is used as
/// default)
pub fn get_jwk_set(private_key_pem: Option<&PathBuf>) -> Result<JwkSet, JwkError> {
    let pem_str = load_private_key_pem_str(private_key_pem)?;
    let key_id = format!("{:x}", md5::compute(pem_str.trim()));
    let jwk_set = jwk_set_from_private_key_pem(&pem_str, key_id)?;
    Ok(jwk_set)
}
