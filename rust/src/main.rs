mod db;
mod leptos_axum;
mod utils;

use axum::{routing::get, Router};
use leptos::view;
use leptos_axum::LeptosHtml;
use tower_http::services::ServeDir;

async fn index() -> LeptosHtml {
    return view! {
<html lang="en">
    <head>
        <title>HTMX Is Neet!</title>
        <meta charset="UTF-8"></meta>
        <meta name="viewport" content="width=device-width, initial-scale=1"></meta>
        <link href="/assets/index.css" rel="stylesheet"></link>
        <script src="https://unpkg.com/htmx.org/dist/htmx.min.js"></script>
        <script src="/assets/bundle.js"></script>
    </head>
    <body>
        <div class="bg-green-100 text-blue-800 w-full h-full">hello, mom</div>
    </body>
</html>
    }.into();
}

#[tokio::main]
async fn main() {
    env_logger::init();

    let app = Router::new()
        .route("/", get(index))
        .nest_service("/assets", ServeDir::new("dist"));

    axum::Server::bind(&"0.0.0.0:42069".parse().unwrap())
        .serve(app.into_make_service())
        .await
        .unwrap();
}
