use anyhow::Result;
use axum::{extract::WebSocketUpgrade, response::Html, routing::get, Router};

use crate::config::CONFIG;

#[allow(non_snake_case)]
mod app;
mod config;
mod external;
mod interface;
mod logic;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: std::net::SocketAddr = CONFIG.host.parse().expect("Invalid 'HOST' env");

    tokio::spawn(async {
        let mut args = vec![
            "run",
            "-A",
            "./scripts/server/app/app.ts",
            CONFIG.deno_server_port,
            &format!("{}/auth.json", CONFIG.config_path),
        ];
        if let Some(lang) = CONFIG.language {
            args.push(lang);
        }
        let _ = tokio::process::Command::new("deno")
            .args(args)
            .spawn()
            .expect("Failed to start deno server")
            .wait()
            .await;
    });

    let view = dioxus_liveview::LiveViewPool::new();

    let app = Router::new()
        // The root route contains the glue code to connect to the WebSocket
        .route(
            "/",
            get(move || async move {
                #[cfg(debug_assertions)]
                let style = "<script src=\"http://localhost:3010\"></script>".to_string();
                #[cfg(not(debug_assertions))]
                let style = format!("<style>{}</style>", include_str!("../style/dist/index.css"));

                Html(format!(
                    r#"
                <!DOCTYPE html>
                <html>
                <head> 
                  <title>Youtube Music Downloader</title>  
                  {style}
                </head>
                <body> <div id="main"></div> </body>
                {glue}
                </html>
                "#,
                    glue = dioxus_liveview::interpreter_glue("/ws")
                ))
            }),
        )
        .route(
            "/ws",
            get(move |ws: WebSocketUpgrade| async move {
                ws.on_upgrade(move |socket| async move {
                    _ = view
                        .launch(dioxus_liveview::axum_socket(socket), app::App)
                        .await;
                })
            }),
        );

    println!("Listening on {addr}");

    let mut sigterm =
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate()).unwrap();

    let server_ =
        axum::Server::bind(&addr.to_string().parse().unwrap()).serve(app.into_make_service());

    tokio::select! {
        _ = server_ => {},
        _ = sigterm.recv() => {},

    }

    Ok(())
}
