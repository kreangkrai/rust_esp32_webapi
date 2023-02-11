use actix_web::{middleware,App,HttpServer,http};
use mee_api::app_config::config_app;
use actix_cors::Cors;

#[actix_web::main]
async fn main() -> std::io::Result<()>{
    std::env::set_var("RUST_LOG", "actix_server=info,actix_web=info");
    env_logger::init();
    
    HttpServer::new(||{
        let cors = Cors::default()
              .allowed_origin("http://172.20.20.3")
              .allowed_origin_fn(|origin, _req_head| {
                  origin.as_bytes().ends_with(b"*")
              })
              .allowed_methods(vec!["GET", "POST","PUT"])
              .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
              .allowed_header(http::header::CONTENT_TYPE)
              .max_age(3600);
        App::new()
        .wrap(cors)
        .wrap(middleware::Logger::default())
        .configure(config_app)    
    })
    .bind(("172.20.10.3",8082))?
    .run()
    .await
}