use png::ColorType;
use axum::response::Response;
use std::io::Cursor;
use axum::response::IntoResponse;
use axum::body::Body;
use axum::extract::State;
use piet::ImageBuf;
use piet::RenderContext;
use piet::ImageFormat;
use piet::InterpolationMode;
use axum::routing::get;
use axum::Router;
use std::collections::HashMap;
use axum::extract::Path;
use axum::http::{StatusCode, header, HeaderName};
use reqwest;
use serde::Deserialize;
use tokio;
use std::env;
use piet::kurbo::Rect;
use piet_common::Device;
use tokio_util::io::ReaderStream;
use png;

#[derive(Clone)]
struct Config {
    lanyard_api: String,
    discord_cdn: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            lanyard_api: env::var("LANYARD_API").expect("Missign API URL"),
            discord_cdn: env::var("DISCORD_CDN").expect("Missign API URL"),
        }
    }
}


#[dotenvy::load(path = "./.env", required = false, override_ = false)]
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("3000".to_owned());
    let addr = format!("{}:{}", host, port);

    let config = Config::default();

    let app = Router::new()
        .route("/{id}", get(card_png))
        .with_state(config);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

#[derive(Deserialize, Debug)]
struct User {
    data: Option<UserData>,
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
struct DiscordUser {
    username: String,
    avatar: String,
    global_name: String,
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
struct UserData {
    active_on_discord_web: bool,
    active_on_discord_mobile: bool,
    active_on_discord_desktop: bool,
    active_on_discord_embedded: bool,
    active_on_discord_vr: bool,
    listening_to_spotify: bool,
    kv: HashMap<String, String>,
    spotify: Option<Spotify>,
    discord_user: DiscordUser,
    #[serde(default)]
    discord_status: String,
    activities: Vec<Activity>,
}


async fn card_png(
    cfg: State<Config>,
    Path(id): Path<String>
) -> Response {
    let user: User = reqwest::get(
            format!("{}/users/{}", cfg.lanyard_api, id)
        )
        .await
        .unwrap() // TODO error handling
        .json()
        .await
        .unwrap();

    let Some(user) = user.data else {
        return (StatusCode::NOT_FOUND).into_response();
    };

    let DiscordUser {
        username,
        avatar,
        global_name
    } = user.discord_user;

    let avatar = reqwest::get(
            format!("{}/avatars/{}/{}.webp?size=64", cfg.discord_cdn, id, avatar)
        )
        .await
        .unwrap()
        .bytes()
        .await
        .unwrap();

    let mut device = Device::new().unwrap();
    let mut target = device.bitmap_target(1024, 1024, 1.0).unwrap();
    let mut ctx = target.render_context();

    let buf = ImageBuf::from_data(&avatar).unwrap();
    let img = buf.to_image(&mut ctx);
    ctx.draw_image(&img, Rect::new(0.0, 0.0, 1024.0, 1024.0), InterpolationMode::Bilinear);
    
    ctx.finish().unwrap();
    let img = target.to_image_buf(ImageFormat::RgbaPremul).unwrap();
    let mut cursor = Cursor::new(img.raw_pixels().to_vec());

    let mut data = vec![0; img.width() * img.height() * 4];
    target.copy_raw_pixels(ImageFormat::RgbaPremul, &mut data).unwrap();
    piet::util::unpremultiply_rgba(&mut data);
    let mut encoder = png::Encoder::new(&mut cursor, img.width() as u32, img.height() as u32);
    encoder.set_color(ColorType::Rgba);
    encoder.set_depth(png::BitDepth::Eight);
    encoder
        .write_header()
        .unwrap()
        .write_image_data(&data)
        .unwrap();

    let headers = [
        (header::CONTENT_TYPE, "image/png"),
    ];

    (headers, cursor.into_inner()).into_response()
}
