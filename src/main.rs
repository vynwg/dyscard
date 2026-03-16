use axum::routing::post;
use axum::routing::get;
use axum::Router;
use reqwest;
use tokio;

use std::env;

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
        .route("/{id}.png", get(card_png))
        .layer(Extension(config));

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await?;

    Ok(())
}

async fn card_png(Path(id): Path<u32>) {

}
