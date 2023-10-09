mod services;

use actix_web::{
    App,
    HttpServer
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(move || {
        App::new()
            .configure(services::config)
    })
    .bind(("127.0.0.1", 3030))?
    .run()
    .await
}
