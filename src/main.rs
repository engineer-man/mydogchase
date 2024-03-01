mod config;
mod routes;
mod media;

use crate::config::read_config;
use crate::routes::app;

use std::env;

#[tokio::main]
async fn main() {
    let config_option = read_config(); if let Some(config) = config_option {
        println!("Address : Port");
        println!("{}:{}", config.ip, config.port);
        let path = env::current_dir().expect("Where am I?");
        println!("Server running in {}", path.display());
        let app = app();
        let addr = format!("{}:{}", config.ip, config.port);
        let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
        let server = axum::serve(listener, app);
        if let Err(e) = server.await {
            eprintln!("Server error: {:?}", e);
        }
    } else {
        println!("Config file not found");
    }
}
