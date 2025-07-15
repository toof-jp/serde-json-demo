use std::fmt::format;

use anyhow::Result;
use serde::{Deserialize, Serialize};

#[tokio::main]
async fn main() -> Result<()> {
    let base_url = "https://reqres.in/api";

    let res = reqwest::Client::new()
        .get(format!("{base_url}/users"))
        .header("x-api-key", "reqres-free-v1")
        .send()
        .await?;

    let users = res.json::<Users>().await?;

    for user in users.data {
        let user_update = UserUpdate { name: format!("Authentic {} {}", user.first_name, user.last_name), job: "Musician".into() };

        let res = reqwest::Client::new()
            .put(format!("{base_url}/users/{}", user.id))
            .header("x-api-key", "reqres-free-v1")
            .json(&user_update)
            .send()
            .await?;
        
        dbg!(res.text().await?);
    }

    Ok(())
}

#[derive(Debug, Deserialize)]
struct Users {
    data: Vec<User>,
}

#[derive(Debug, Deserialize)]
struct User {
    id: i64,
    email: String,
    first_name: String,
    last_name: String,
    avatar: String,
}

#[derive(Serialize)]
struct UserUpdate {
    name: String,
    job: String,
}
