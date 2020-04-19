mod kv;
mod kv_handler;


#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    use actix_web::{
        middleware::Logger,
        web::{scope, resource, get, post, delete},
        App,
        HttpServer
    };

    let db: kv::RocksDB = kv::KVStore::init("/tmp/rocks/actix-db");

    std::env::set_var("RUST_LOG", "actix_web=info,actix_server=info");
    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .data(db.clone())
            .wrap(Logger::default())
            .service(
                scope("/api")
                .service(
                    resource("/{key}")
                        .route(get().to(kv_handler::get))
                        .route(post().to(kv_handler::post))
                        .route(delete().to(kv_handler::delete)),
                ),
            )
    })
    .bind("0.0.0.0:8080")?
    .run()
    .await
}
