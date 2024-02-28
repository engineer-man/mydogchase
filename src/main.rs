mod config;
mod routes;
mod utils;
mod media;
mod constants;
mod generate;
pub mod solarized;

use crate::config::read_config;
use crate::routes::app;
use crate::generate::*;

use std::env;
use axum_server::{self, tls_rustls::RustlsConfig};
use solarized::{
    print_colored, print_fancy, clear,
    VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA,
    WHITE,
    BOLD, UNDERLINED, ITALIC,
    PrintMode::NewLine,
};

#[tokio::main]
async fn main() {
    clear();
    let args: Vec<String> = env::args().collect();
    if args.contains(&"-h".to_string()) || args.contains(&"--help".to_string()) {
        print_fancy(&[
            ("This program is designed to be a modular web service.\n", CYAN, vec![]),
            ("All html and media paths are read from the ", CYAN, vec![]),
            ("config.toml", VIOLET, vec![]),
            (" file.\n", CYAN, vec![]),
            ("If ", CYAN, vec![]),
            ("config.toml", VIOLET, vec![]),
            (" does not exist, an example project structure can be created.\n", CYAN, vec![]),
            ("The config.toml file should contain something similar to the following.\n", CYAN, vec![]),

            ("\nip", BLUE, vec![]),
            (" = ", WHITE, vec![]),
            ("\"0.0.0.0\"\n", CYAN, vec![]),

            ("port", BLUE, vec![]),
            (" = ", WHITE, vec![]),
            ("8000\n", CYAN, vec![]),

            ("ssl_enabled", BLUE, vec![]),
            (" = ", WHITE, vec![]),
            ("false\n", WHITE, vec![]),

            ("ssl_port", BLUE, vec![]),
            (" = ", WHITE, vec![]),
            ("8443\n", CYAN, vec![]),

            ("ssl_cert_path", BLUE, vec![]),
            (" = ", WHITE, vec![]),
            ("\"pems/cert.pem\"\n", CYAN, vec![]),

            ("ssl_key_path", BLUE, vec![]),
            (" = ", WHITE, vec![]),
            ("\"pems/key.pem\"\n", CYAN, vec![]),

            ("\n[routes]\n", ORANGE, vec![]),

            ("\"/\"", BLUE, vec![]),
            (" = [", WHITE, vec![]),
            ("\"templates/home/home.html\"", CYAN, vec![]),
            (", ", WHITE, vec![]),
            ("\"public/chase\"", CYAN, vec![]),
            ("]\n", WHITE, vec![]),

            ("\"/\"", BLUE, vec![]),
            (" = [", WHITE, vec![]),
            ("\"templates/home/home.html\"", CYAN, vec![]),
            ("]\n", WHITE, vec![]),
        ], NewLine);
        return;
    }
    print_colored(
        &["R", "a", "i", "n", "b", "o", "w", "s"],
        &[VIOLET, BLUE, CYAN, GREEN, YELLOW, ORANGE, RED, MAGENTA],
        NewLine
    );
    let config_option = read_config(); if let Some(config) = config_option {
        print_fancy(&[
            ("config.yml ", CYAN, vec![]),
            ("found", GREEN, vec![]),
        ], NewLine);
        if config.ssl_enabled {
            print_fancy(&[
                ("\nSSL", GREEN, vec![]),
                (" is ", CYAN, vec![]),
                ("Enabled\n", GREEN, vec![]),
                ("\nAddress : Port\n", CYAN, vec![BOLD, ITALIC, UNDERLINED]),
                (&format!("{}", config.ip), BLUE, vec![]),
                (":", CYAN, vec![BOLD]),
                (&format!("{}\n", config.ssl_port), VIOLET, vec![]),
                (&format!("https://{}:{}\n", config.ip, config.ssl_port), GREEN, vec![BOLD, ITALIC, UNDERLINED]),
            ], NewLine);
        } else {
            print_fancy(&[
                ("\nSSL", YELLOW, vec![]),
                (" is ", CYAN, vec![]),
                ("NOT", RED, vec![BOLD, ITALIC]),
                (" Enabled\n", ORANGE, vec![]),
                ("\nAddress : Port\n", CYAN, vec![BOLD, ITALIC, UNDERLINED]),
                (&format!("{}", config.ip), BLUE, vec![]),
                (":", CYAN, vec![BOLD]),
                (&format!("{}\n", config.port), VIOLET, vec![]),
                (&format!("http://{}:{}", config.ip, config.port), GREEN, vec![BOLD, ITALIC, UNDERLINED]),
            ], NewLine);
        }
        print_fancy(&[
            ("\nHardcoded routes:\n", CYAN, vec![BOLD, ITALIC, UNDERLINED]),
            ("/", BLUE, vec![]),
            (" -> ", CYAN, vec![]),
            ("root", VIOLET, vec![]),
        ], NewLine);
        print_fancy(&[
            ("\nConfigured routes:", CYAN, vec![BOLD, ITALIC, UNDERLINED]),
        ], NewLine);
        for (path, settings) in &config.routes {
            let file = settings.get(0)
                .map(|s| s.to_string())
                .unwrap_or_else(|| "No file specified".to_string());
            let media_info = if settings.len() > 1 {
                format!("{}", settings[1])
            } else {
                "".to_string()
            };
            print_fancy(&[
                (&format!("{}", path), BLUE, vec![]),
                (" -> ", CYAN, vec![]),
                (&format!("{}", &file), VIOLET, vec![]),
                (" -> ", CYAN, vec![]),
                (&format!("{}", &media_info), MAGENTA, vec![]),
            ], NewLine);
        }
        let path = env::current_dir().expect("asdf");
        print_fancy(&[
            ("\nServer running in ", CYAN, vec![]),
            (&format!("{}", path.display()), VIOLET, vec![]),
        ], NewLine);
        if config.ssl_enabled {
            let app = app(&config);
            let ssl_config = RustlsConfig::from_pem_file(
                config.ssl_cert_path.expect("SSL cert path is required"),
                config.ssl_key_path.expect("SSL key path is required"),
            ).await.expect("Failed to configure SSL");
            let addr = format!("{}:{}", config.ip, config.ssl_port);
            let server = axum_server::bind_rustls(addr.parse().unwrap(), ssl_config)
                .serve(app.into_make_service());
            server.await.unwrap();
        } else {
            let app = app(&config);
            let addr = format!("{}:{}", config.ip, config.port);
            let listener = tokio::net::TcpListener::bind(&addr).await.unwrap();
            let server = axum::serve(listener, app);
            if let Err(e) = server.await {
                eprintln!("Server error: {:?}", e);
            }
        }
    } else {
        generate_files();
    }
}
