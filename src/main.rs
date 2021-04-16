use actix_web::{web, App, HttpServer};

mod handlers;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=debug");

    HttpServer::new(move || {
        App::new()
            .route("/units", web::get().to(handlers::get_units))
            .route("/units/{id}", web::get().to(handlers::get_unit_by_id))
            .route("/units", web::post().to(handlers::add_unit))
            .route("/units/{id}", web::delete().to(handlers::delete_unit))
            .route("/units/{id}", web::put().to(handlers::update_unit))
    })
    .bind("127.0.0.1:8001")
    .run()
    .await
}
