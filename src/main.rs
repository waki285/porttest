use std::net::SocketAddr;

use axum::{extract::ConnectInfo, response::Html, routing::get, Router};

#[tokio::main]
async fn main() {
    let args: Vec<String> = std::env::args().collect();
    let port = args.get(1).unwrap_or(&"3000".to_string()).parse::<u16>().unwrap();

    let app = Router::new().route("/", get(handler));

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", port))
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());
    axum::serve(listener, app.into_make_service_with_connect_info::<SocketAddr>()).await.unwrap();
}

async fn handler(ConnectInfo(addr): ConnectInfo<SocketAddr>) -> Html<String> {
    let html = format!(
        r#"
        <h1>Hello, world!</h1>
        <p>Your IP address is: {}</p>
    "#,
        addr
    );
    Html(html)
}