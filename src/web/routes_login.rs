use crate::api_errors::Error;
use anyhow::Result;
use axum::{Json, Router};
use axum::routing::post;
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use tower_cookies::{Cookies, Cookie};
#[derive(Debug, Deserialize, Serialize)]
pub struct LoginPayload{
    user_name: String,
    password: String
}
impl LoginPayload {
    pub fn new(user_name: &str, password: &str) -> Self {
        LoginPayload {
            user_name: user_name.to_string(),
            password: password.to_string(),
        }
    }
}

pub fn routes() -> Router {
    Router::new().route("/api/login", post(api_login))
}

async fn api_login(cookies: Cookies, payload: Json<LoginPayload>) ->  Result<Json<Value>, Error> {
    println!("->> {:<12} - api_login", "HANDLER");

    //TODO : Implement real db/auth logic.
    if payload.user_name != "demo1" || payload.password != "welcome" {
        return Err(Error::LoginFail);
    }

    // TODO: Set cookies
    cookies.add(Cookie::new("aut-token", "user-1.exp.sign"));

    // Create the succes body

    let body = Json(json!({
        "result": {
            "success":  true
        }
    }));
    
    Ok(body)
}


