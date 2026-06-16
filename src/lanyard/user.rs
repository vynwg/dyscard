use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug)]
pub struct User {
    pub data: Option<UserData>,
}

#[derive(Deserialize, Debug)]
struct Timestamp {
    start: Option<u64>,
    end: Option<u64>,
}

#[derive(Deserialize, Debug)]
struct Spotify {
    track_id: String,
    timestamps: Timestamp,
    song: String,
    artist: String,
    album_art_url: String,
    album: String,
}

#[derive(Deserialize, Debug)]
pub struct DiscordUser {
    pub username: String,
    pub avatar: String,
    pub global_name: String,
}

#[derive(Deserialize, Debug)]
struct Emoji {
    name: String,
    id: Option<String>,
    #[serde(default)]
    animated: bool,
}

#[derive(Deserialize, Debug)]
struct Assets {
    large_image: Option<String>,
    large_text: Option<String>,
    small_image: Option<String>,
    small_text: Option<String>,
}

#[derive(Deserialize, Debug)]
struct Activity {
    name: String,
    r#type: u8,
    created_at: u64,
    timestamps: Option<Timestamp>,
    #[serde(default)]
    application_id: String,
    #[serde(default)]
    details: String,
    #[serde(default)]
    state: String,
    emoji: Option<Emoji>,
    assets: Option<Assets>,
}

#[derive(Deserialize, Debug)]
pub struct UserData {
    active_on_discord_web: bool,
    active_on_discord_mobile: bool,
    active_on_discord_desktop: bool,
    active_on_discord_embedded: bool,
    active_on_discord_vr: bool,
    listening_to_spotify: bool,
    kv: HashMap<String, String>,
    spotify: Option<Spotify>,
    pub discord_user: DiscordUser,
    #[serde(default)]
    discord_status: String,
    activities: Vec<Activity>,
}
