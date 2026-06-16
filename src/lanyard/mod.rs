#[allow(dead_code)]
pub mod user;

use user::User;
use anyhow::Result;

pub async fn get_user(endpoint: String) -> Result<User> {
    let user: User = reqwest::get(endpoint)
        .await?
        .json()
        .await?;

    Ok(user)
}
