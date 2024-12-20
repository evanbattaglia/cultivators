use crate::lti_models::dynamic_registration::DynRegLtiMessage;
use crate::lti_models::platform_openid_configuration::PlatformOpenidLtiMessage;

impl From<PlatformOpenidLtiMessage> for DynRegLtiMessage {
    fn from(platform_supported_message: PlatformOpenidLtiMessage) -> Self {
        DynRegLtiMessage {
            message_type: platform_supported_message.message_type,
            placements: platform_supported_message.placements,
            target_link_uri: None,
            label: None,
        }
    }
}
