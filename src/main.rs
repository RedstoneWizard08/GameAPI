use actix_web::{guard, web, App, HttpServer};

mod routes;
mod util;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(routes::game_versions::versions)
            .service(routes::loader_versions::loader)
            .service(routes::game_download::download)
            .service(routes::loader_download::download_loader)
            .default_service(
                web::route()
                    .guard(guard::Not(guard::Get()))
                    .to(routes::default::error),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
