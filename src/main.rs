#![recursion_limit = "1024"]

//! Static site generator for legopal.fr.
//!
//! `cargo run --release` renders every route to plain HTML in `dist/` and
//! copies `public/` next to it. The stylesheet is built separately by the
//! Tailwind CLI (see `package.json`). `cargo run --release -- serve` previews
//! `dist/` locally the way GitHub Pages serves it.

use gopal::app::*;
use leptos::prelude::*;
use leptos_axum::generate_route_list_with_ssg;
use std::path::Path;

const SITE_ROOT: &str = "dist";
const ASSETS_DIR: &str = "public";
const PREVIEW_ADDR: &str = "127.0.0.1:3000";

#[tokio::main]
async fn main() {
    match std::env::args().nth(1).as_deref() {
        None | Some("build") => build().await,
        Some("serve") => serve().await,
        Some(other) => {
            eprintln!("unknown command `{other}`, expected `build` or `serve`");
            std::process::exit(2);
        }
    }
}

async fn build() {
    let options = LeptosOptions::builder()
        .output_name("gopal")
        .site_root(SITE_ROOT)
        .build();

    // Keep the stylesheet the Tailwind CLI may already have written.
    let css = Path::new(SITE_ROOT).join("pkg");
    for entry in std::fs::read_dir(SITE_ROOT).into_iter().flatten().flatten() {
        let path = entry.path();
        if path != css {
            if path.is_dir() {
                std::fs::remove_dir_all(&path).unwrap();
            } else {
                std::fs::remove_file(&path).unwrap();
            }
        }
    }
    copy_dir(Path::new(ASSETS_DIR), Path::new(SITE_ROOT)).unwrap();

    let (_routes, static_routes) = generate_route_list_with_ssg(shell);
    static_routes.generate(&options).await;

    println!("site written to {SITE_ROOT}/");
}

/// Serves `dist/` like GitHub Pages: `/menu` resolves to `menu.html` and
/// unknown paths get `404.html`.
async fn serve() {
    use axum::{extract::Request, middleware::map_request, Router};
    use tower_http::services::{ServeDir, ServeFile};

    async fn add_html_extension(mut req: Request) -> Request {
        let path = req.uri().path();
        if path != "/" && !path.ends_with('/') && Path::new(path).extension().is_none() {
            *req.uri_mut() = format!("{path}.html").parse().unwrap();
        }
        req
    }

    let files = ServeDir::new(SITE_ROOT)
        .not_found_service(ServeFile::new(Path::new(SITE_ROOT).join("404.html")));
    let app = Router::new()
        .fallback_service(files)
        .layer(map_request(add_html_extension));

    println!("serving {SITE_ROOT}/ on http://{PREVIEW_ADDR}");
    let listener = tokio::net::TcpListener::bind(PREVIEW_ADDR).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

fn copy_dir(from: &Path, to: &Path) -> std::io::Result<()> {
    std::fs::create_dir_all(to)?;
    for entry in std::fs::read_dir(from)? {
        let entry = entry?;
        let target = to.join(entry.file_name());
        if entry.file_type()?.is_dir() {
            copy_dir(&entry.path(), &target)?;
        } else if entry.file_name() != ".DS_Store" {
            std::fs::copy(entry.path(), target)?;
        }
    }
    Ok(())
}
