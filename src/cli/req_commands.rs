use super::args::Req;
use super::util::run_async_block;
use crate::app_routes;
use crate::crypto;
use crate::seaorm_setup::create_db_connection;
use color_eyre::eyre;

pub fn main(req: Req) -> eyre::Result<()> {
    use super::args::Req::*;

    match req {
        Login(req_login) => {
            crypto::files::ensure_tool_keys_loaded();
            let bogus_payload = app_routes::login::LoginPayload {
                lti_message_hint: "TODO-insert-real-lti-message-hint-here".to_string(),
                login_hint: "TODO-insert-real-login-hint-here".to_string(),
                lti_storage_target: "TODO-insert-real-lti-storage-target-here".to_string(),
                client_id: req_login.client_id, // could just get from registration
            };
            let bogus_path = app_routes::login::LoginPath {
                registration_uuid: req_login.registration_uuid,
            };
            let res = run_async_block(async {
                // TODO get only one (see other useage of this in CLI)
                let db = create_db_connection().await?;
                let login = app_routes::login::_login(&db, bogus_path, bogus_payload).await?;
                Ok(login)
            })?;
            println!("{}", serde_json::to_string_pretty(&res)?);
            Ok(())
        }
    }
}
