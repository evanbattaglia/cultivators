//! This module is responsible for building the Dynamic Registration JSON for the tool. The JSON is
//! sent to the platform in the Dynamic Registration process.

use serde::Deserialize;
use serde::Serialize;
use std::collections::HashMap;

// LTI constant values
const APPLICATION_TYPE: &str = "web";
const RESPONSE_TYPES: [&str; 1] = ["id_token"];
const GRANT_TYPES: [&str; 2] = ["implicit", "client_credentials"];
const TOKEN_ENDPOINT_AUTH_METHOD: &str = "private_key_jwt";
// claims can vary, but this is good enough for our Tool
const CLAIMS: [&str; 2] = ["iss", "sub"];
const MINIMUM_SCOPES: [&str; 1] = [
    "openid",
    // TODO canvas complains about this?! I thought we had to have it
    // "https://purl.imsglobal.org/spec/lti-reg/scope/registration"
];

// Spec-detailed optional values (and i18n variants) not implemented in our Tool:
// * logo_uri
// * contacts
// * tos_uri
// * policy_uri

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct DynRegLtiMessage {
    #[serde(rename = "type")]
    pub message_type: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub target_link_uri: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub label: Option<String>, // not implemented: i18n -- label#ja etc.
    #[serde(default)]
    pub placements: Vec<String>,
}

/// Parameters to to create a Dynamic Registration JSON blob for a Tool, as specified by the
/// 1EdTech spec.
/// Values not specified in this blob are hardcoded to constant values (in the case of mandatory
/// values) or not implemented here.
#[derive(Serialize, Clone, Debug)]
pub struct DynRegParams {
    /// URL to kick off the LTI 1.3 launch
    pub initiate_login_uri: String,
    /// Valid redirect_uris (launch URLs for target_link_uri)
    pub redirect_uris: Vec<String>,

    /// URL at which the Tool serves its JWKs (public keys)
    pub jwks_uri: String,

    /// Tool name.
    /// Note: Spec allows for i18n with properties such as "client_name#ja"; these are not
    /// implemented in our Tool.
    pub client_name: String,

    /// Scopes (permissions)
    pub scopes: Vec<String>,

    /// Tool domain, used in LTI config
    pub domain: String,

    /// Tool description. Note: spec allows for i18n e.g. "description#ja"; these are not implement
    /// in our Tool.
    pub description: String,

    /// default launch URL
    pub target_link_uri: String,

    /// LTI custom parameters to be (possibly) expanded by Platform and included in launches
    pub custom_parameters: HashMap<String, String>,

    /// messages supported by the Tool
    pub messages: Vec<DynRegLtiMessage>,
}

// Struct reflecting structure specified by 1EdTech Spec
#[derive(Serialize, Debug)]
pub struct DynamicRegistration {
    application_type: &'static str,
    response_types: &'static [&'static str],
    grant_types: &'static [&'static str],
    initiate_login_uri: String,
    redirect_uris: Vec<String>,
    client_name: String,
    jwks_uri: String,
    token_endpoint_auth_method: &'static str,
    scope: String,
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-tool-configuration")]
    lti_config: DynamicRegistrationLtiConfig,
}

// Struct used in above DynamicRegistration private struct
#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicRegistrationLtiConfig {
    domain: String,
    description: String,
    target_link_uri: String,
    custom_parameters: HashMap<String, String>,
    messages: Vec<DynRegLtiMessage>,
    claims: Vec<String>,
}

pub fn build_registration_json(params: DynRegParams) -> DynamicRegistration {
    let DynRegParams {
        initiate_login_uri,
        redirect_uris,
        jwks_uri,
        client_name,
        scopes,
        domain,
        description,
        target_link_uri,
        custom_parameters,
        messages,
    } = params;

    let mut scopes: Vec<String> = scopes.clone();
    scopes.extend(MINIMUM_SCOPES.iter().map(|a| a.to_string()));

    DynamicRegistration {
        application_type: APPLICATION_TYPE,
        initiate_login_uri,
        response_types: &RESPONSE_TYPES,
        grant_types: &GRANT_TYPES,
        redirect_uris,
        client_name,
        jwks_uri,
        token_endpoint_auth_method: TOKEN_ENDPOINT_AUTH_METHOD,
        scope: scopes.join(" "),
        lti_config: DynamicRegistrationLtiConfig {
            domain,
            description,
            target_link_uri,
            custom_parameters,
            messages,
            claims: CLAIMS.iter().map(|a| a.to_string()).collect(),
        },
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct DynamicRegistrationResponse {
    pub application_type: String,
    pub client_id: String,
    pub client_name: String,
    pub grant_types: Vec<String>,
    #[serde(rename = "https://purl.imsglobal.org/spec/lti-tool-configuration")]
    pub lti_config: DynamicRegistrationLtiConfig,
    pub initiate_login_uri: String,
    pub jwks_uri: String,
    pub logo_uri: Option<String>,
    pub redirect_uris: Vec<String>,
    pub response_types: Vec<String>,
    pub scope: String,
    pub token_endpoint_auth_method: String,
}
