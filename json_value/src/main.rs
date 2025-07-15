use anyhow::Result;
use serde_json::{Value, json};

#[tokio::main]
async fn main() -> Result<()> {
    let client = reqwest::Client::new();
    let base_url = "https://reqres.in/api";
    let res = client
        .get(format!("{base_url}/users"))
        .header("x-api-key", "reqres-free-v1")
        .send()
        .await?
        .text()
        .await?;

    let json: Value = serde_json::from_str(&res)?;

    for user in json["data"]
        .as_array()
        .expect("json[\"data\"] should be Value::Array")
    {
        let id = user["id"]
            .as_i64()
            .expect("user[\"id\"] should be Value::Number");

        let body = json!({
            "name": format!("Authentic {} {}", user["first_name"].as_str().unwrap(), user["last_name"].as_str().unwrap()),
        });

        let res = client
            .put(format!("{base_url}/users/{id}"))
            .header("x-api-key", "reqres-free-v1")
            .json(&body)
            .send()
            .await?
            .text()
            .await?;

        dbg!(res);
    }

    Ok(())
}
