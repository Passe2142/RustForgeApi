#![allow(unused)]
use std::collections::HashMap;

use reqwest::header::SET_COOKIE;
use rust_web_backend_project::web::routes_login::LoginPayload;
use axum::response::Json;
use reqwest::Response;
use serde_json::json;
use serde_json::Value;

async fn get_request(
    client: &reqwest::Client,
    url: &str,
) -> Result<reqwest::Response, reqwest::Error> {
    // Make a GET request
    let response: reqwest::Response = client.get(url).send().await?;
    Ok(response)
}
async fn post_request(
    client: &reqwest::Client,
    url: &str,
    body: Value,
) -> Result<reqwest::Response, reqwest::Error> {
    // Make a POST request with the specified JSON body
    let response: reqwest::Response = client.post(url).json(&body).send().await?;
    Ok(response)
}

#[tokio::test]
async fn quick_dev() -> Result<(), Box<dyn std::error::Error>> {
    // Replace the base URL with the address of your local server
    let base_url: &str = "http://localhost:8080";

    // Create a reqwest client with the base URL
    let client: reqwest::Client = reqwest::Client::new();

    // Define endpoints to request
    let endpoints: Vec<&str> = vec!["hello2/jules" ];

    let req_response = post_request(
            &client,
        "http://localhost:8080/api/login",
        json!({
            "user_name": "demo1",
            "password": "welcome"
        }),
    ).await?
    ;

    // Iterate over endpoints
    for endpoint in endpoints {
        // Make a GET request for the current endpoint
        let response = get_request(&client, &format!("{}/{}", base_url, endpoint)).await?;

        // Example: Print the desired display
        println!("=== Response for GET {}", response.url());
        println!("=> Status\t: {}", response.status());

        // Print headers
        println!("=> Headers\t:");
        for (name, value) in response.headers().iter() {
            println!("   {}\t: {:?}", name, value);
        }

        // Print response body
        let body = response.text().await?;
        println!("=> Response Body for {}:", endpoint);
        println!("{}", body);

        println!("===\n");
    }
        // Print display for POST
        println!("=== Response for POST {}", req_response.url());
        println!("=> Status\t: {}", req_response.status());
    
        // Print headers
        println!("=> Headers\t:");
        for (name, value) in req_response.headers().iter() {
            println!("   {}\t: {:?}", name, value);
        }

    // Print Response Cookies from POST
    if let Some(cookies_header) = req_response.headers().get_all(SET_COOKIE).iter().next() {
        if let Ok(cookies_str) = cookies_header.to_str() {
            println!("=> Response Cookies\t:");
            for cookie in cookies_str.split(';') {
                println!("   {}", cookie.trim());
            }
        }
    }

    
    
        // Print response body
        let body = req_response.text().await?;
        println!("=> Response Body for POST:");
        println!("{}", body);
            
        println!("===\n");
    

    Ok(())
}

