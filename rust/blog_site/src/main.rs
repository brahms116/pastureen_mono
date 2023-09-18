use axum::{
    http::HeaderMap,
    response::{Html, IntoResponse, Response},
    routing::get,
    Router,
};
use components::LayoutProps;

use std::net::SocketAddr;

#[tokio::main]
async fn main() {
    let app = Router::new()
        .route("/", get(index))
        .route("/static/logo.png", get(logo));

    let addr: SocketAddr = "0.0.0.0:8080".parse().unwrap();

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn index() -> Response {
    let layout_props = LayoutProps {
        title: "Pastureen",
        navbar_props: components::NavbarProps {
            logo_link: "/",
            logo_src: "/static/logo.png",
            logo_text: "Pastureen",
            nav_items: &[
                components::NavItemProps {
                    link: "/",
                    text: "Home",
                    is_active: true,
                },
                components::NavItemProps {
                    link: "/about",
                    text: "About",
                    is_active: false,
                },
            ],
        },
    };
    Html(components::layout(layout_props).into_string()).into_response()
}

const LOGO: &[u8] = include_bytes!("../assets/logo.png");

async fn logo() -> Response {
    let mut headers = HeaderMap::new();
    headers.insert("content-type", "image/png".parse().unwrap());
    (headers, LOGO).into_response()
}
