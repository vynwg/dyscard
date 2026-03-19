use axum::routing::post;
use axum::routing::get;
use axum::Extension;
use axum::Router;
use axum::response::IntoResponse;
use axum::extract::Path;
use reqwest;
use tokio;

use std::env;

#[derive(Clone)]
struct Config {
    lanyard_api: String
}

impl Default for Config {
    fn default() -> Self {
        Config {
            lanyard_api: env::var("LANYARD_API").expect("Missign API URL")
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
        .layer(Extension(config));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

struct User {
    success: bool,
    data: UserData
}

struct Timestamp {
    start: u64,
    end: u64
}

struct Spotify {
    track_id: String,
    timestamps: Timestamp,
    song: String,
    artist: String,
    album_art_url: String,
    album: String
}

struct DiscordUser {
    username: String,
    public_flags: u32,
    id: String,
    discriminator: String,
    avatar: String
}

struct Activity {
    type: u8,
    timestamps: Timestampm,
    sync_id: String,
    "state": "Ark Patrol; Veronika Redd",
        "session_id": "140ecdfb976bdbf29d4452d492e551c7",
        "party": {
          "id": "spotify:94490510688792576"
        },
        "name": "Spotify",
        "id": "spotify:1",
        "flags": 48,
        "details": "Let Go",
        "created_at": 1615529838051,
        "assets": {
          "large_text": "Let Go",
          "large_image": "spotify:ab67616d0000b27364840995fe43bb2ec73a241d"
        }
}

struct UserData {
    active_on_discord_mobile: bool,
    active_on_discord_desktop: bool,
    listening_to_spotify: bool,
    kv: HashMap<String, String>,
    spotify: Spotify,
    discord_user: DiscordUser,
    discord_status: String,
    activities: Vec<Activity>
    [
      {
        "type": 2,
        "timestamps": {
          "start": 1615529820677,
          "end": 1615530068733
        },
        "sync_id": "3kdlVcMVsSkbsUy8eQcBjI",
        "state": "Ark Patrol; Veronika Redd",
        "session_id": "140ecdfb976bdbf29d4452d492e551c7",
        "party": {
          "id": "spotify:94490510688792576"
        },
        "name": "Spotify",
        "id": "spotify:1",
        "flags": 48,
        "details": "Let Go",
        "created_at": 1615529838051,
        "assets": {
          "large_text": "Let Go",
          "large_image": "spotify:ab67616d0000b27364840995fe43bb2ec73a241d"
        }
      },
      {
        "type": 0,
        "timestamps": {
          "start": 1615438153941
        },
        "state": "Workspace: lanyard",
        "name": "Visual Studio Code",
        "id": "66b84f5317e9de6c",
        "details": "Editing README.md",
        "created_at": 1615529838050,
        "assets": {
          "small_text": "Visual Studio Code",
          "small_image": "565945770067623946",
          "large_text": "Editing a MARKDOWN file",
          "large_image": "565945077491433494"
        },
        "application_id": "383226320970055681"
      }
    ]
  }
}

async fn card_png(
    Path(id): Path<String>,
    Extension(cfg): Extension<Config>
) -> impl IntoResponse {
    let url = format!("{}/users/{}", cfg.lanyard_api, id);

    let res = reqwest::get(url)
        .await
        .unwrap() // TODO error handling
        .text()
        .await
        .unwrap();

    println!("{:?}", res);
}
