use std::env;

use anyhow::Result;
use axum::{extract::WebSocketUpgrade, response::Html, routing::get, Router};

#[allow(non_snake_case)]
mod app;
mod constants;
mod external;
mod interface;
mod logic;

#[tokio::main]
async fn main() -> Result<()> {
    let addr: std::net::SocketAddr = env::var("HOST")
        .expect("Env 'HOST' not found")
        .parse()
        .expect("Invalid 'HOST' env");

    tokio::spawn(async {
        let _ = tokio::process::Command::new("deno")
            .args(["run", "-A", "./scripts/app.ts"])
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

    axum::Server::bind(&addr.to_string().parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();

    Ok(())
}
