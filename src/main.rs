mod lanyard;

use lanyard::get_user;
use lanyard::user::DiscordUser;

use anyhow::{Result, Error};
use png::ColorType;
use axum::response::Response;
use std::io::Cursor;
use axum::response::IntoResponse;
use axum::extract::State;
use piet_common::ImageBuf;
use piet_common::RenderContext;
use piet_common::ImageFormat;
use piet_common::InterpolationMode;
use axum::routing::get;
use axum::Router;
use axum::extract::Path;
use axum::http::{StatusCode, header};
use reqwest;
use tokio;
use std::env;
use piet_common::kurbo::Rect;
use piet_common::Device;
use png;

#[derive(Clone)]
struct Config {
    lanyard_api: String,
    discord_cdn: String,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            lanyard_api: env::var("LANYARD_API").unwrap_or("https://lanyard.vynwg.com/v1".to_owned()),
            discord_cdn: env::var("DISCORD_CDN").unwrap_or("https://cdn.discordapp.com".to_owned()),
        }
    }
}

#[derive(Clone)]
struct Endpoints {
    discord_avatar: String,
    lanyard_user: String,
}


impl Endpoints {
    fn load(cfg: Config) -> Self {
        Self {
            discord_avatar: format!("{}/avatars", cfg.discord_cdn),
            lanyard_user: format!("{}/users", cfg.lanyard_api),
        }
    }

    fn avatar(&self, id: String, hash: String, ext: &'static str, size: u64) -> String {
        format!("{}/{}/{}.{}?size={}", self.discord_avatar, id, hash, ext, size)
    }

    fn lanyard(&self, id: String) -> String {
        format!("{}/{}", self.lanyard_user, id)
    }
}


#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let host = env::var("HOST").unwrap_or("0.0.0.0".to_owned());
    let port = env::var("PORT").unwrap_or("3000".to_owned());
    let addr = format!("{}:{}", host, port);

    let config = Config::default();
    let endpoints = Endpoints::load(config);

    let app = Router::new()
        .route("/{id}", get(card_png))
        .with_state(endpoints);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    axum::serve(listener, app).await?;

    Ok(())
}


async fn get_img(endpoint: String) -> Result<ImageBuf> {
    let avatar = reqwest::get(endpoint)
        .await?
        .bytes()
        .await?;

    let buf = ImageBuf::from_data(&avatar)
        .map_err(Error::from_boxed)?;

    Ok(buf)
}

async fn card_png(
    edp: State<Endpoints>,
    Path(id): Path<String>
) -> Response {
    
    let user = get_user(edp.lanyard(id.clone())).await.unwrap();

    let Some(user) = user.data else {
        return (StatusCode::NOT_FOUND).into_response();
    };

    let DiscordUser { avatar, .. } = user.discord_user;

    let buf = get_img(edp.avatar(id, avatar, "webp", 1024)).await.unwrap();

    let mut device = Device::new().unwrap();
    let mut target = device.bitmap_target(1024, 1024, 1.0).unwrap();
    let mut ctx = target.render_context();

    let img = buf.to_image(&mut ctx);
    ctx.draw_image(&img, Rect::new(0.0, 0.0, 1024.0, 1024.0), InterpolationMode::Bilinear);
    
    ctx.finish().unwrap();
    let img = target.to_image_buf(ImageFormat::RgbaPremul).unwrap();
    let mut cursor = Cursor::new(img.raw_pixels().to_vec());

    let mut data = vec![0; img.width() * img.height() * 4];
    target.copy_raw_pixels(ImageFormat::RgbaPremul, &mut data).unwrap();
    piet_common::util::unpremultiply_rgba(&mut data);
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
