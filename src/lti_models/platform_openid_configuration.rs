use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformOpenidConfiguration {
    pub issuer: String,
    pub authorization_endpoint: String,
    pub registration_endpoint: String,
    pub jwks_uri: String,
    pub token_endpoint: String,
    pub token_endpoint_auth_methods_supported: Vec<String>,
    pub token_endpoint_auth_signing_alg_values_supported: Vec<String>,
    pub scopes_supported: Vec<String>,
    pub response_types_supported: Vec<String>,
    pub id_token_signing_alg_values_supported: Vec<String>,
    pub claims_supported: Vec<String>,
    pub subject_types_supported: Vec<String>,
    pub authorization_server: Option<String>,
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-platform-configuration")]
    pub lti_configuration: PlatformOpenidLtiConfiguration,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformOpenidLtiMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    pub placements: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct PlatformOpenidLtiConfiguration {
    pub product_family_code: String,
    pub version: String,
    pub messages_supported: Vec<PlatformOpenidLtiMessage>,
    #[serde(default)]
    pub variables: Vec<String>,

    // Canvas Extensions
    #[serde(rename = "https://canvas.instructure.com/lti/account_name")]
    pub account_name: Option<String>,
    #[serde(rename = "https://canvas.instructure.com/lti/account_lti_guid")]
    pub account_lti_guid: Option<String>,
}
