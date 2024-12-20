use thiserror::Error;

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("Failed to generate RSA private key")]
    RsaPrivateKeyGenerationError(#[from] rsa::errors::Error),
    #[error("Failed to encode RSA private key")]
    RsaPrivateKeyEncodingError(#[from] rsa::pkcs8::Error),
}


