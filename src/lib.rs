/// Web app routes
pub mod app_routes;

/// Generation of Web App URIs, used outside of web app too (e.g. generation of Dynamic
/// Registration is available through the CLI)
pub mod app_uris;

pub mod app;
pub mod cli;

pub mod crypto;
pub mod env;
pub mod seaorm_setup;

/// Models (struct / JSON structures) to be used for an LTI;
/// Not really specific to _this_ LTI tool with its configuration
pub mod lti_models;

/// Abstraction over lti_models to dal with the LTI models
/// with Cultivators-specific configuration.
pub mod lti_app_models;
