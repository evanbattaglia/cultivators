use crate::lti_models::dynamic_registration::DynRegLtiMessage;
use crate::lti_models::platform_openid_configuration::PlatformOpenidLtiMessage;
use crate::app_uris;
use std::collections::HashMap;

impl From<PlatformOpenidLtiMessage> for DynRegLtiMessage {
    fn from(platform_supported_message: PlatformOpenidLtiMessage) -> Self {
        let mut custom_parameters = HashMap::new();
        custom_parameters.insert(
            "cultivators_msg_type".to_string(),
            platform_supported_message.message_type.clone(),
        );
        let target_link_uri = Some(
            app_uris::launch_uri_with_message_type(&platform_supported_message.message_type)
        );
        DynRegLtiMessage {
            message_type: platform_supported_message.message_type,
            placements: platform_supported_message.placements,
            target_link_uri,
            label: None,
            custom_parameters,
        }
    }
}
