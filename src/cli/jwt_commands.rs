use super::args::{CliJwt, CliJwtVal};
use crate::cli::eyre::eyre;
use color_eyre::eyre;
use jsonwebtoken::TokenData;
use jsonwebtoken::{jwk::JwkSet, Algorithm, DecodingKey, Validation};

pub fn cli_jwt_val(cmd: CliJwtVal) -> eyre::Result<()> {
    // TODO use private key
    let filename = cmd
        .key_file
        .ok_or(eyre!("TODO implement using key from tool"))?;
    let file_contents = std::fs::read_to_string(filename)?;
    let jwkset: JwkSet = serde_json::from_str(&file_contents)?;

    let jwt_filename = cmd.jwt_file.ok_or(eyre!("TODO implement STDIN"))?;
    let jwt = std::fs::read_to_string(jwt_filename)?;
    let jwt = jwt.trim();
    for (i, jwk) in jwkset.keys.iter().enumerate() {
        println!(
            "=== Trying with JWK number {} {}",
            i,
            jwk.common.key_id.as_deref().unwrap_or("")
        );
        let pubkey = DecodingKey::from_jwk(jwk)?;
        let mut alg = Validation::new(Algorithm::RS256);
        if cmd.audience.is_empty() {
            alg.validate_aud = false;
        } else {
            alg.set_audience(&[&cmd.audience]);
        }
        if cmd.ignore_expiry {
            alg.validate_exp = false;
        }
        match jsonwebtoken::decode::<serde_json::Value>(jwt, &pubkey, &alg) {
            Ok(val) => {
                let TokenData { header, claims } = val;
                println!("JWT valid!");
                println!("Header: {}", serde_json::to_string_pretty(&header)?);
                println!("Claims: {}", serde_json::to_string_pretty(&claims)?);
            }
            Err(err) => {
                println!("Decode failed with error: {:?}", err);
            }
        }
    }
    Ok(())
}

pub fn main(jwt_cli: CliJwt) -> eyre::Result<()> {
    use CliJwt::*;
    match jwt_cli {
        Val(cmd) => cli_jwt_val(cmd),
    }
}
