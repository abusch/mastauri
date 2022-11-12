use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use url::Url;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum Visibility {
    Public,
    Unlisted,
    Private,
    Direct,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Attachment {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Application {
    pub name: String,
    pub website: Option<String>,
    pub vapid_key: Option<String>,
    // TODO client attributes
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Mention {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub url: Url,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct History {
    pub day: String,
    pub uses: String,
    pub accounts: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Tag {
    pub name: String,
    pub url: String,
    #[serde(default)]
    pub history: Vec<History>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Emoji {
    pub shortcode: String,
    pub url: String,
    pub static_url: String,
    pub visible_in_picker: Option<bool>,
    pub category: Option<String>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum CardType {
    Link,
    Photo,
    Video,
    Rich,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Card {
    pub url: String,
    pub title: String,
    pub description: String,
    pub r#type: CardType,
    pub author_name: Option<String>,
    pub author_url: Option<String>,
    pub provider_name: Option<String>,
    pub provider_url: Option<String>,
    pub html: Option<String>,
    pub width: Option<u64>,
    pub height: Option<u64>,
    pub image: Option<String>,
    pub embed_url: Option<String>,
    pub blurhash: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Poll {}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub username: String,
    pub acct: String,
    pub url: Url,
    pub display_name: String,
    pub note: String,
    pub avatar: Url,
    pub avatar_static: Url,
    pub header: Url,
    pub header_static: Url,
    #[serde(default)]
    pub locked: bool,
    pub emojis: Vec<Emoji>,
    #[serde(default)]
    pub discoverable: bool,
    // statistical attributes
    pub created_at: String,     // DateTime<Utc>,
    pub last_status_at: String, //DateTime<Utc>,
    #[serde(default)]
    pub statuses_count: u64,
    #[serde(default)]
    pub followers_count: u64,
    #[serde(default)]
    pub following_count: u64,
    // TODO optional attributes
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Status {
    // base attributes
    pub id: String,
    pub uri: Url,
    pub created_at: DateTime<Utc>,
    pub account: Account,
    pub content: String,
    pub visibility: Visibility,
    #[serde(default)]
    pub sensitive: bool,
    pub spoiler_text: String,
    pub media_attachments: Vec<Attachment>,
    pub application: Option<Application>,

    // rendering attributes
    pub mentions: Vec<Mention>,
    pub tags: Vec<Tag>,
    pub emojis: Vec<Emoji>,

    // informational attributes
    #[serde(default)]
    pub reblogs_count: u64,
    #[serde(default)]
    pub favourites_count: u64,
    #[serde(default)]
    pub replies: u64,

    // nullable attributes
    pub url: Option<Url>,
    pub in_reply_to_id: Option<String>,
    pub in_reply_to_account_id: Option<String>,
    pub reblog: Option<Box<Status>>,
    pub poll: Option<Poll>,
    pub card: Option<Card>,
    pub language: Option<String>,
    pub text: Option<String>,
}
