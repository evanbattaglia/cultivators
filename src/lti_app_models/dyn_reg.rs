use crate::app_uris;
use crate::lti_models::dynamic_registration::build_registration_json;
use crate::lti_models::dynamic_registration::DynRegLtiMessage;
use crate::lti_models::dynamic_registration::DynRegParams;
use crate::lti_models::dynamic_registration::DynamicRegistration;
use std::collections::HashMap;

pub const CLIENT_NAME: &str = "Cultivators Rust LTI Tool";
pub const CLIENT_DESCRIPTION: &str = "Cultivators Rust LTI Tool";

pub fn get_dynamic_registration(
    registration_id: &str,
    messages: Vec<DynRegLtiMessage>,
    scopes: Vec<String>,
) -> DynamicRegistration {
    let params = DynRegParams {
        initiate_login_uri: app_uris::login_uri(registration_id),
        redirect_uris: vec![app_uris::launch_uri()],
        jwks_uri: app_uris::jwks_uri(),
        client_name: CLIENT_NAME.to_string(),
        scopes,
        domain: crate::env::base_url_domain(),
        description: CLIENT_DESCRIPTION.to_string(),
        target_link_uri: app_uris::launch_uri(),
        custom_parameters: HashMap::new(),
        messages,
    };
    build_registration_json(params)
}

// can be used in instances (like dyn reg generation in CLI) where there is no applicable Platform
// to base messages and scopes off of
pub fn get_dynamic_registration_default_messages(registration_id: &str) -> DynamicRegistration {
    let message = DynRegLtiMessage {
        message_type: "LtiResourceLinkRequest".to_string(),
        placements: None,
        target_link_uri: None,
        label: None,
        custom_parameters: HashMap::new(),
    };
    get_dynamic_registration(registration_id, vec![message], vec![])
}
