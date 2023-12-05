#![allow(unused)]

use rust_web_backend_project::api_errors::{Error, Result};
use axum::response::{Html, IntoResponse, Response};
use axum::routing::{get, get_service};
use axum::{Router, handler, middleware};
use rust_web_backend_project::web;
use serde::Deserialize;
use axum::extract::{Query, Path};
use tower_cookies::CookieManagerLayer;
use tower_http::services::ServeDir;
use std::net::SocketAddr;



#[tokio::main]
async fn main() {
    let routes_all: Router = Router::new()
    .merge(routes_hello())
    .merge(web::routes_login::routes())
    .layer(middleware::map_response(main_response_mapper))
    .layer(CookieManagerLayer::new())
    .fallback_service(routes_static());

    // region:  Start Server
    let address: SocketAddr = SocketAddr::from(([127, 0 , 0, 1], 8080));
    println!("->> Listening on {address}\n");


    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind(&address).await.unwrap();
    axum::serve(listener, routes_all)
    .await
    .unwrap();
    // endregion: End Server
}

async fn main_response_mapper(res: Response) ->  Response {
    println!("->> {:<12} - main_response_mapper", "RES_MAPPER");
    println!();
    res
}


fn routes_static() -> Router{
    Router::new().nest_service("/", get_service(ServeDir::new("./")))
}


// region:  --- Routes hello
fn routes_hello() -> Router { 
    Router::new()
    .route("/hello",get(handler_hello))
    .route("/hello2/:name", get(handler_hello2))
}
#[derive(Debug, Deserialize)]
struct HelloParams {
    name: Option<String>,
}

async fn handler_hello(Query(params): Query<HelloParams>) -> impl IntoResponse{
    println!("->> {:<12} - handler_hello - {params:?}", "HANDLER");
    let name = params.name.as_deref().unwrap_or("World!");
    Html(format!("Hello <strong>{name}</strong>"))
}

async fn handler_hello2(Path(name): Path<String>) -> impl IntoResponse{

    println!("->> {:<12} - handler_hello - {name:?}", "HANDLER");

    Html(format!("Hello <strong>{name}</strong>"))

}

// endregion:  --- Routes hello
