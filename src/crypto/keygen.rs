use rsa::pkcs8::{EncodePrivateKey, LineEnding};
use rsa::RsaPrivateKey;
use thiserror::Error;
use zeroize::Zeroizing;

#[derive(Error, Debug)]
pub enum KeygenError {
    #[error("Failed to generate RSA private key")]
    RsaPrivateKeyGenerationError(#[from] rsa::errors::Error),
    #[error("Failed to encode RSA private key")]
    RsaPrivateKeyEncodingError(#[from] rsa::pkcs8::Error),
}

pub fn generate_rsa_private_key() -> Result<Zeroizing<String>, KeygenError> {
    let mut rng = rand::thread_rng();
    let bits = 2048;
    let priv_key = RsaPrivateKey::new(&mut rng, bits)?;
    Ok(priv_key.to_pkcs8_pem(LineEnding::LF)?)
}
