use reqwest::{Client, Error, StatusCode};
use reqwest::header::{ AUTHORIZATION, CONTENT_TYPE };
use std::env;

#[tokio::main]
async fn main() -> Result<(), Error> {
    unsafe {
        env::set_var("API_TOKEN", "");
    }
    let api_token = env::var("API_TOKEN")
        .expect("Api token environment variable not set");
    let droplet_id = "";
    let client = Client::new();


    let res = client.get("https://api.digitalocean.com/v2/account")
        .header(AUTHORIZATION, format!("Bearer {}", api_token))
        .header(CONTENT_TYPE, "application/json")
        .send()
        .await?;
    if res.status().is_success() {
        println!("API authentication successful!");
        let body = res.text().await?;
        println!("Account details: {}", body);
    } else {
        println!("API authentication failed: {:?}", res.status());
        let error_body = res.text().await?;
        eprintln!("Error details: {}", error_body);
    }

    
    let url = format!("https://api.digitalocean.com/v2/droplets/{}", droplet_id);

    let response = client.delete(&url)
        .header("Authorization", format!("Bearer {}", api_token))
        .send().await?;

    match response.status() {
        StatusCode::NO_CONTENT => {
            println!("Droplet {} is destroyed.", droplet_id);
        }
        _ => {
            eprintln!("Failed to destroy Droplet {}. Status: {:?}", droplet_id, response.status());
            eprintln!("Response body: {:?}", response.text().await?);
        }
    }
    
    Ok(())
}
