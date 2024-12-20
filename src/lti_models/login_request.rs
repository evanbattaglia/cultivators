//! This module is responsible for building the URL that the User Agent will be redirected to in
//! the login_request handler. This URL is the Platform's Authorization Endpoint, with the required
//! parameters

use serde::Serialize;

/// Parameters passed into this module to build the URL
#[derive(Serialize)]
pub struct LaunchParams<'a> {
    pub lti_message_hint: &'a str,
    pub login_hint: &'a str,
    pub state: &'a str,
    pub nonce: &'a str,
}

/// These values are potentially varying per registration, platform, or deployment
pub struct DeploymentParams<'a> {
    /// Platform's URL for the next step of the launch
    pub platform_authorize_uri: &'a str,

    /// our developer key ID with the platform
    pub client_id: &'a str,

    /// redirect uri / target_link_uri that we want the Platform to launch us at
    pub redirect_uri: &'a str,
}

// Private struct to hold the parameters that will be serialized into the URL query string
#[derive(Serialize)]
struct UrlQueryParams<'a> {
    // LTI constant values
    prompt: &'static str,
    response_mode: &'static str,
    response_type: &'static str,
    scope: &'static str,

    // Potentially varying per platform/registration
    client_id: &'a str,
    redirect_uri: &'a str,

    // Values varying per launch:
    #[serde(flatten)]
    launch_params: &'a LaunchParams<'a>,
}

// LTI constant values
const RESPONSE_TYPE: &str = "id_token";
const RESPONSE_MODE: &str = "form_post";
const SCOPE: &str = "openid";
const PROMPT: &str = "none";

/// Construct a URL on the Platform's site corresponding to Step 3, the Authentication step. This
/// URL is to be used during Step 2, the Authentication Request, to forward the User Agent from the
/// tool back to the platform. See
/// <https://www.imsglobal.org/spec/security/v1p1#step-2-authentication-request-0> and
/// <https://www.imsglobal.org/spec/lti/v1p3#additional-login-parameters>
pub fn build_url(
    deployment_params: &DeploymentParams,
    launch_params: &LaunchParams,
) -> Result<String, serde_urlencoded::ser::Error> {
    let query_params = serde_urlencoded::to_string(UrlQueryParams {
        prompt: PROMPT,
        response_mode: RESPONSE_MODE,
        response_type: RESPONSE_TYPE,
        scope: SCOPE,

        redirect_uri: deployment_params.redirect_uri,
        client_id: deployment_params.client_id,

        launch_params,
    })?;

    Ok(format!(
        "{}?{}",
        deployment_params.platform_authorize_uri, query_params
    ))
}
