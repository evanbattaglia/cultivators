use crate::cli::args::Gen;
use crate::crypto::keygen::generate_rsa_private_key;
use crate::lti_app_models::dyn_reg::get_dynamic_registration_default_messages;
use color_eyre::eyre;

pub fn main(gen: Gen) -> eyre::Result<()> {
    match gen {
        Gen::Key => {
            eprintln!("Generating RSA private key...");
            let priv_pem_str = generate_rsa_private_key()?;
            print!("{}", priv_pem_str.as_str());
        }
        Gen::JwkSet(gen_jwk) => {
            eprintln!("Generating JWK from RSA private key...");
            let path_buf = gen_jwk.private_key.map(std::path::PathBuf::from);
            let jwk = crate::crypto::files::get_jwk_set(path_buf.as_ref())?;
            serde_json::to_writer(std::io::stdout(), &jwk)?;
        }
        Gen::DynReg(_gen_dyn_reg) => {
            let bogus_reg_id = "TODO-insert-real-reg-uuid-here";
            let dr = get_dynamic_registration_default_messages(bogus_reg_id);
            println!("{}", serde_json::to_string_pretty(&dr)?);
        }
    }
    Ok(())
}
