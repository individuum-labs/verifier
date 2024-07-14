use serde::Deserialize;
use serde::Serialize;
use serde_json::Value;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Root {
    pub data: Data,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Data {
    #[serde(rename = "threaded_conversation_with_injections_v2")]
    pub threaded_conversation_with_injections_v2: ThreadedConversationWithInjectionsV2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ThreadedConversationWithInjectionsV2 {
    pub instructions: Vec<Instruction>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Instruction {
    #[serde(rename = "type")]
    pub type_field: String,
    pub entries: Option<Vec<Entry>>,
    pub direction: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entry {
    pub entry_id: String,
    pub sort_index: String,
    pub content: Content,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Content {
    pub entry_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    pub item_content: Option<ItemContent>,
    #[serde(default)]
    pub items: Vec<Value>,
    pub display_type: Option<String>,
    pub header: Option<Header>,
    pub client_event_info: Option<ClientEventInfo>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ItemContent {
    pub item_type: String,
    #[serde(rename = "__typename")]
    pub typename: String,
    #[serde(rename = "tweet_results")]
    pub tweet_results: TweetResults,
    pub tweet_display_type: String,
    pub has_moderated_replies: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TweetResults {
    pub result: Result,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result {
    #[serde(rename = "__typename")]
    pub typename: String,
    #[serde(rename = "rest_id")]
    pub rest_id: String,
    #[serde(rename = "has_birdwatch_notes")]
    pub has_birdwatch_notes: bool,
    pub core: Core,
    #[serde(rename = "unmention_data")]
    pub unmention_data: UnmentionData,
    #[serde(rename = "edit_control")]
    pub edit_control: EditControl,
    #[serde(rename = "is_translatable")]
    pub is_translatable: bool,
    pub views: Views,
    pub source: String,
    pub legacy: Legacy2,
    #[serde(rename = "quick_promote_eligibility")]
    pub quick_promote_eligibility: QuickPromoteEligibility,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Core {
    #[serde(rename = "user_results")]
    pub user_results: UserResults,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResults {
    pub result: Result2,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Result2 {
    #[serde(rename = "__typename")]
    pub typename: String,
    pub id: String,
    #[serde(rename = "rest_id")]
    pub rest_id: String,
    #[serde(rename = "affiliates_highlighted_label")]
    pub affiliates_highlighted_label: AffiliatesHighlightedLabel,
    #[serde(rename = "has_graduated_access")]
    pub has_graduated_access: bool,
    #[serde(rename = "is_blue_verified")]
    pub is_blue_verified: bool,
    #[serde(rename = "profile_image_shape")]
    pub profile_image_shape: String,
    pub legacy: Legacy,
    #[serde(rename = "tipjar_settings")]
    pub tipjar_settings: TipjarSettings,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AffiliatesHighlightedLabel {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legacy {
    #[serde(rename = "can_dm")]
    pub can_dm: bool,
    #[serde(rename = "can_media_tag")]
    pub can_media_tag: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "default_profile")]
    pub default_profile: bool,
    #[serde(rename = "default_profile_image")]
    pub default_profile_image: bool,
    pub description: String,
    pub entities: Entities,
    #[serde(rename = "fast_followers_count")]
    pub fast_followers_count: i64,
    #[serde(rename = "favourites_count")]
    pub favourites_count: i64,
    #[serde(rename = "followers_count")]
    pub followers_count: i64,
    #[serde(rename = "friends_count")]
    pub friends_count: i64,
    #[serde(rename = "has_custom_timelines")]
    pub has_custom_timelines: bool,
    #[serde(rename = "is_translator")]
    pub is_translator: bool,
    #[serde(rename = "listed_count")]
    pub listed_count: i64,
    pub location: String,
    #[serde(rename = "media_count")]
    pub media_count: i64,
    pub name: String,
    #[serde(rename = "needs_phone_verification")]
    pub needs_phone_verification: bool,
    #[serde(rename = "normal_followers_count")]
    pub normal_followers_count: i64,
    #[serde(rename = "pinned_tweet_ids_str")]
    pub pinned_tweet_ids_str: Vec<Value>,
    #[serde(rename = "possibly_sensitive")]
    pub possibly_sensitive: bool,
    #[serde(rename = "profile_banner_url")]
    pub profile_banner_url: String,
    #[serde(rename = "profile_image_url_https")]
    pub profile_image_url_https: String,
    #[serde(rename = "profile_interstitial_type")]
    pub profile_interstitial_type: String,
    #[serde(rename = "screen_name")]
    pub screen_name: String,
    #[serde(rename = "statuses_count")]
    pub statuses_count: i64,
    #[serde(rename = "translator_type")]
    pub translator_type: String,
    pub url: String,
    pub verified: bool,
    #[serde(rename = "want_retweets")]
    pub want_retweets: bool,
    #[serde(rename = "withheld_in_countries")]
    pub withheld_in_countries: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities {
    pub description: Description,
    pub url: Url,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Description {
    pub urls: Vec<Value>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url {
    pub urls: Vec<Url2>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Url2 {
    #[serde(rename = "display_url")]
    pub display_url: String,
    #[serde(rename = "expanded_url")]
    pub expanded_url: String,
    pub url: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct TipjarSettings {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UnmentionData {}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EditControl {
    #[serde(rename = "edit_tweet_ids")]
    pub edit_tweet_ids: Vec<String>,
    #[serde(rename = "editable_until_msecs")]
    pub editable_until_msecs: String,
    #[serde(rename = "is_edit_eligible")]
    pub is_edit_eligible: bool,
    #[serde(rename = "edits_remaining")]
    pub edits_remaining: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Views {
    pub count: String,
    pub state: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Legacy2 {
    #[serde(rename = "bookmark_count")]
    pub bookmark_count: i64,
    pub bookmarked: bool,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "conversation_id_str")]
    pub conversation_id_str: String,
    #[serde(rename = "display_text_range")]
    pub display_text_range: Vec<i64>,
    pub entities: Entities2,
    #[serde(rename = "favorite_count")]
    pub favorite_count: i64,
    pub favorited: bool,
    #[serde(rename = "full_text")]
    pub full_text: String,
    #[serde(rename = "is_quote_status")]
    pub is_quote_status: bool,
    pub lang: String,
    #[serde(rename = "quote_count")]
    pub quote_count: i64,
    #[serde(rename = "reply_count")]
    pub reply_count: i64,
    #[serde(rename = "retweet_count")]
    pub retweet_count: i64,
    pub retweeted: bool,
    #[serde(rename = "user_id_str")]
    pub user_id_str: String,
    #[serde(rename = "id_str")]
    pub id_str: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Entities2 {
    pub hashtags: Vec<Value>,
    pub symbols: Vec<Value>,
    pub timestamps: Vec<Value>,
    pub urls: Vec<Value>,
    #[serde(rename = "user_mentions")]
    pub user_mentions: Vec<UserMention>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UserMention {
    #[serde(rename = "id_str")]
    pub id_str: String,
    pub name: String,
    #[serde(rename = "screen_name")]
    pub screen_name: String,
    pub indices: Vec<i64>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct QuickPromoteEligibility {
    pub eligibility: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Header {
    pub display_type: String,
    pub text: String,
    pub social_context: SocialContext,
    pub sticky: bool,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SocialContext {
    #[serde(rename = "type")]
    pub type_field: String,
    pub context_type: String,
    pub text: String,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ClientEventInfo {
    pub component: String,
    pub details: Details,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Details {
    pub conversation_details: ConversationDetails,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ConversationDetails {
    pub conversation_section: String,
}
