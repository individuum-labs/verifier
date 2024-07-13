use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub protected: bool,
    #[serde(rename = "screen_name")]
    pub screen_name: String,
    #[serde(rename = "always_use_https")]
    pub always_use_https: bool,
    #[serde(rename = "use_cookie_personalization")]
    pub use_cookie_personalization: bool,
    #[serde(rename = "sleep_time")]
    pub sleep_time: SleepTime,
    #[serde(rename = "geo_enabled")]
    pub geo_enabled: bool,
    pub language: String,
    #[serde(rename = "discoverable_by_email")]
    pub discoverable_by_email: bool,
    #[serde(rename = "discoverable_by_mobile_phone")]
    pub discoverable_by_mobile_phone: bool,
    #[serde(rename = "display_sensitive_media")]
    pub display_sensitive_media: bool,
    #[serde(rename = "personalized_trends")]
    pub personalized_trends: bool,
    #[serde(rename = "allow_media_tagging")]
    pub allow_media_tagging: String,
    #[serde(rename = "allow_contributor_request")]
    pub allow_contributor_request: String,
    #[serde(rename = "allow_ads_personalization")]
    pub allow_ads_personalization: bool,
    #[serde(rename = "allow_logged_out_device_personalization")]
    pub allow_logged_out_device_personalization: bool,
    #[serde(rename = "allow_location_history_personalization")]
    pub allow_location_history_personalization: bool,
    #[serde(rename = "allow_sharing_data_for_third_party_personalization")]
    pub allow_sharing_data_for_third_party_personalization: bool,
    #[serde(rename = "allow_dms_from")]
    pub allow_dms_from: String,
    #[serde(rename = "always_allow_dms_from_subscribers")]
    pub always_allow_dms_from_subscribers: Value,
    #[serde(rename = "allow_dm_groups_from")]
    pub allow_dm_groups_from: String,
    #[serde(rename = "translator_type")]
    pub translator_type: String,
    #[serde(rename = "country_code")]
    pub country_code: String,
    #[serde(rename = "nsfw_user")]
    pub nsfw_user: bool,
    #[serde(rename = "nsfw_admin")]
    pub nsfw_admin: bool,
    #[serde(rename = "ranked_timeline_setting")]
    pub ranked_timeline_setting: i64,
    #[serde(rename = "ranked_timeline_eligible")]
    pub ranked_timeline_eligible: Value,
    #[serde(rename = "address_book_live_sync_enabled")]
    pub address_book_live_sync_enabled: bool,
    #[serde(rename = "universal_quality_filtering_enabled")]
    pub universal_quality_filtering_enabled: String,
    #[serde(rename = "dm_receipt_setting")]
    pub dm_receipt_setting: String,
    #[serde(rename = "alt_text_compose_enabled")]
    pub alt_text_compose_enabled: Value,
    #[serde(rename = "mention_filter")]
    pub mention_filter: String,
    #[serde(rename = "allow_authenticated_periscope_requests")]
    pub allow_authenticated_periscope_requests: bool,
    #[serde(rename = "protect_password_reset")]
    pub protect_password_reset: bool,
    #[serde(rename = "require_password_login")]
    pub require_password_login: bool,
    #[serde(rename = "requires_login_verification")]
    pub requires_login_verification: bool,
    #[serde(rename = "ext_sharing_audiospaces_listening_data_with_followers")]
    pub ext_sharing_audiospaces_listening_data_with_followers: bool,
    pub ext: Ext,
    #[serde(rename = "dm_quality_filter")]
    pub dm_quality_filter: String,
    #[serde(rename = "autoplay_disabled")]
    pub autoplay_disabled: bool,
    #[serde(rename = "settings_metadata")]
    pub settings_metadata: SettingsMetadata,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SleepTime {
    pub enabled: bool,
    #[serde(rename = "end_time")]
    pub end_time: Value,
    #[serde(rename = "start_time")]
    pub start_time: Value,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ext {
    pub sso_connections: SsoConnections,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SsoConnections {
    pub r: R,
    pub ttl: i64,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct R {
    pub ok: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SettingsMetadata {
    #[serde(rename = "is_eu")]
    pub is_eu: String,
}
