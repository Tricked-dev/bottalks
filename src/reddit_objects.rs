/// Generated by https://quicktype.io
extern crate serde_json;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
#[derive(Serialize, Deserialize)]
pub struct NewestJson {
    #[serde(rename = "data")]
    pub data: NewestJsonData,
}

#[derive(Serialize, Deserialize)]
pub struct NewestJsonData {
    #[serde(rename = "children")]
    pub children: Vec<Child>,
}

#[derive(Serialize, Deserialize)]
pub struct Child {
    #[serde(rename = "data")]
    pub data: ChildData,
}

#[derive(Serialize, Deserialize)]
pub struct ChildData {
    #[serde(rename = "url")]
    pub url: String,
}

#[derive(Serialize, Deserialize)]
pub struct AllAwarding {
    #[serde(rename = "giver_coin_reward")]
    pub giver_coin_reward: Option<i64>,

    #[serde(rename = "subreddit_id")]
    pub subreddit_id: Option<serde_json::Value>,

    #[serde(rename = "is_new")]
    pub is_new: bool,

    #[serde(rename = "days_of_drip_extension")]
    pub days_of_drip_extension: i64,

    #[serde(rename = "coin_price")]
    pub coin_price: i64,

    #[serde(rename = "id")]
    pub id: String,

    #[serde(rename = "penny_donate")]
    pub penny_donate: Option<i64>,

    #[serde(rename = "award_sub_type")]
    pub award_sub_type: String,

    #[serde(rename = "coin_reward")]
    pub coin_reward: i64,

    #[serde(rename = "icon_url")]
    pub icon_url: String,

    #[serde(rename = "days_of_premium")]
    pub days_of_premium: i64,

    #[serde(rename = "tiers_by_required_awardings")]
    pub tiers_by_required_awardings: Option<serde_json::Value>,

    #[serde(rename = "resized_icons")]
    pub resized_icons: Vec<ResizedIcon>,

    #[serde(rename = "icon_width")]
    pub icon_width: i64,

    #[serde(rename = "static_icon_width")]
    pub static_icon_width: i64,

    #[serde(rename = "start_date")]
    pub start_date: Option<serde_json::Value>,

    #[serde(rename = "is_enabled")]
    pub is_enabled: bool,

    #[serde(rename = "awardings_required_to_grant_benefits")]
    pub awardings_required_to_grant_benefits: Option<serde_json::Value>,

    #[serde(rename = "description")]
    pub description: String,

    #[serde(rename = "end_date")]
    pub end_date: Option<serde_json::Value>,

    #[serde(rename = "subreddit_coin_reward")]
    pub subreddit_coin_reward: i64,

    #[serde(rename = "count")]
    pub count: i64,

    #[serde(rename = "static_icon_height")]
    pub static_icon_height: i64,

    #[serde(rename = "name")]
    pub name: String,

    #[serde(rename = "resized_static_icons")]
    pub resized_static_icons: Vec<ResizedIcon>,

    #[serde(rename = "icon_format")]
    pub icon_format: Option<String>,

    #[serde(rename = "icon_height")]
    pub icon_height: i64,

    #[serde(rename = "penny_price")]
    pub penny_price: Option<i64>,

    #[serde(rename = "award_type")]
    pub award_type: String,

    #[serde(rename = "static_icon_url")]
    pub static_icon_url: String,
}

#[derive(Serialize, Deserialize)]
pub struct ResizedIcon {
    #[serde(rename = "url")]
    pub url: String,

    #[serde(rename = "width")]
    pub width: i64,

    #[serde(rename = "height")]
    pub height: i64,
}

#[derive(Serialize, Deserialize)]
pub struct FlairRichtext {
    #[serde(rename = "e")]
    pub e: String,

    #[serde(rename = "t")]
    pub t: String,
}

#[derive(Serialize, Deserialize)]
pub struct GalleryData {
    #[serde(rename = "items")]
    pub items: Vec<Item>,
}

#[derive(Serialize, Deserialize)]
pub struct Item {
    #[serde(rename = "media_id")]
    pub media_id: String,

    #[serde(rename = "id")]
    pub id: i64,

    #[serde(rename = "caption")]
    pub caption: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Gildings {
    #[serde(rename = "gid_1")]
    pub gid_1: Option<i64>,
}

#[derive(Serialize, Deserialize)]
pub struct Media {
    #[serde(rename = "reddit_video")]
    pub reddit_video: RedditVideo,
}

#[derive(Serialize, Deserialize)]
pub struct RedditVideo {
    #[serde(rename = "bitrate_kbps")]
    pub bitrate_kbps: i64,

    #[serde(rename = "fallback_url")]
    pub fallback_url: String,

    #[serde(rename = "height")]
    pub height: i64,

    #[serde(rename = "width")]
    pub width: i64,

    #[serde(rename = "scrubber_media_url")]
    pub scrubber_media_url: String,

    #[serde(rename = "dash_url")]
    pub dash_url: String,

    #[serde(rename = "duration")]
    pub duration: i64,

    #[serde(rename = "hls_url")]
    pub hls_url: String,

    #[serde(rename = "is_gif")]
    pub is_gif: bool,

    #[serde(rename = "transcoding_status")]
    pub transcoding_status: String,
}

#[derive(Serialize, Deserialize)]
pub struct MediaEmbed {}

#[derive(Serialize, Deserialize)]
pub struct MediaMetadatum {}

#[derive(Serialize, Deserialize)]
pub struct S {
    #[serde(rename = "y")]
    pub y: i64,

    #[serde(rename = "x")]
    pub x: i64,

    #[serde(rename = "u")]
    pub u: String,
}

#[derive(Serialize, Deserialize)]
pub struct Preview {
    #[serde(rename = "images")]
    pub images: Vec<Image>,

    #[serde(rename = "enabled")]
    pub enabled: bool,
}

#[derive(Serialize, Deserialize)]
pub struct Image {
    #[serde(rename = "source")]
    pub source: ResizedIcon,

    #[serde(rename = "resolutions")]
    pub resolutions: Vec<ResizedIcon>,

    #[serde(rename = "variants")]
    pub variants: MediaEmbed,

    #[serde(rename = "id")]
    pub id: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum Edited {
    Bool(bool),

    Double(f64),
}

/// Generated by https://quicktype.io

pub type PostData = Vec<PostDatum>;

#[derive(Serialize, Deserialize)]
pub struct PostDatum {
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "data")]
    pub data: PostDatumData,
}

#[derive(Serialize, Deserialize)]
pub struct PostDatumData {
    #[serde(rename = "after")]
    pub after: Option<serde_json::Value>,

    #[serde(rename = "dist")]
    pub dist: Option<i64>,

    #[serde(rename = "modhash")]
    pub modhash: String,

    #[serde(rename = "geo_filter")]
    pub geo_filter: String,

    #[serde(rename = "children")]
    pub children: Vec<PurpleChild>,

    #[serde(rename = "before")]
    pub before: Option<serde_json::Value>,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleChild {
    #[serde(rename = "kind")]
    pub kind: String,

    #[serde(rename = "data")]
    pub data: PurpleData,
}

#[derive(Serialize, Deserialize)]
pub struct PurpleData {
    #[serde(rename = "media_metadata")]
    pub media_metadata: Option<HashMap<String, MediaMetadatum>>,

    #[serde(rename = "send_replies")]
    pub send_replies: bool,

    #[serde(rename = "replies")]
    pub replies: Option<RepliesUnion>,

    #[serde(rename = "collapsed_reason_code")]
    pub collapsed_reason_code: Option<serde_json::Value>,

    #[serde(rename = "parent_id")]
    pub parent_id: Option<String>,

    #[serde(rename = "collapsed")]
    pub collapsed: Option<bool>,

    #[serde(rename = "body")]
    pub body: Option<String>,

    #[serde(rename = "body_html")]
    pub body_html: Option<String>,
}

#[derive(Serialize, Deserialize)]
pub struct Source {}

#[derive(Serialize, Deserialize)]
pub struct RepliesClass {
    #[serde(rename = "data")]
    pub data: RepliesData,
}

#[derive(Serialize, Deserialize)]
pub struct RepliesData {
    #[serde(rename = "children")]
    pub children: Vec<FluffyChild>,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyChild {
    #[serde(rename = "data")]
    pub data: FluffyData,
}

#[derive(Serialize, Deserialize)]
pub struct FluffyData {
    #[serde(rename = "body")]
    pub body: String,
}

#[derive(Serialize, Deserialize)]
#[serde(untagged)]
pub enum RepliesUnion {
    RepliesClass(RepliesClass),

    String(String),
}
