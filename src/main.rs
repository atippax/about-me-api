use axum::{response::Json, routing::get, Router};
use axum_hello_world::aboutme::aboutme::{Aboutme, aboutme};
async fn get_about_me() -> Json<Aboutme> {
    let aboutme = aboutme();
    Json(aboutme)
}

#[shuttle_runtime::main]
async fn main() -> shuttle_axum::ShuttleAxum {
    let router = Router::new().route("/", get(get_about_me));

    Ok(router.into())
}
