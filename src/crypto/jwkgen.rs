use base64::{engine::general_purpose::URL_SAFE, Engine};
use jsonwebtoken::jwk::AlgorithmParameters::RSA;
use jsonwebtoken::jwk::Jwk;
use jsonwebtoken::jwk::JwkSet;
use jsonwebtoken::jwk::KeyAlgorithm;
use jsonwebtoken::jwk::PublicKeyUse;
use jsonwebtoken::jwk::{CommonParameters, RSAKeyParameters, RSAKeyType};
use jsonwebtoken::DecodingKey;
use jsonwebtoken::EncodingKey;

use rsa::pkcs8::DecodePrivateKey;
use rsa::traits::PublicKeyParts;
use rsa::RsaPrivateKey;
use rsa::RsaPublicKey;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum JwkGenError {
    #[error("Failed to encode RSA public key from private key")]
    PrivateToPublicConversionError(#[from] rsa::pkcs8::spki::Error),
    #[error("Failed to decode RSA private key (rsa crate)")]
    PrivateKeyDecodingError(#[from] rsa::pkcs8::Error),
    #[error("Failed to decode RSA private key (jsonwebtoken crate)")]
    PrivateKeyEncodingError(#[from] jsonwebtoken::errors::Error),
}

pub fn encoding_key_from_private_pem(private_key: &str) -> Result<EncodingKey, JwkGenError> {
    let res = EncodingKey::from_rsa_pem(private_key.as_bytes())?;
    Ok(res)
}

pub fn decoding_key_from_private_pem(private_key: &str) -> Result<DecodingKey, JwkGenError> {
    let pub_key = rsa_pub_key_from_private_pem(private_key)?;
    let n = pub_key.n().to_bytes_be();
    let e = pub_key.e().to_bytes_be();
    Ok(DecodingKey::from_rsa_raw_components(&n, &e))
}

fn rsa_pub_key_from_private_pem(private_key: &str) -> Result<RsaPublicKey, JwkGenError> {
    let priv_key = RsaPrivateKey::from_pkcs8_pem(private_key)?;
    Ok(RsaPublicKey::from(&priv_key))
}

pub fn jwk_from_private_key_pem(private_key: &str, key_id: String) -> Result<Jwk, JwkGenError> {
    let pub_key = rsa_pub_key_from_private_pem(private_key)?;
    let rsa_params = RSAKeyParameters {
        key_type: RSAKeyType::RSA,
        n: URL_SAFE.encode(pub_key.n().to_bytes_be()),
        e: URL_SAFE.encode(pub_key.e().to_bytes_be()),
    };
    let common = CommonParameters {
        public_key_use: Some(PublicKeyUse::Signature),
        key_operations: None,
        key_algorithm: Some(KeyAlgorithm::RS256),
        key_id: Some(key_id),
        x509_url: None,
        x509_chain: None,
        x509_sha1_fingerprint: None,
        x509_sha256_fingerprint: None,
    };
    Ok(Jwk {
        common,
        algorithm: RSA(rsa_params),
    })
}

pub fn jwk_set_from_private_key_pem(
    private_key: &str,
    key_id: String,
) -> Result<JwkSet, JwkGenError> {
    let jwk = jwk_from_private_key_pem(private_key, key_id)?;
    Ok(JwkSet { keys: vec![jwk] })
}
