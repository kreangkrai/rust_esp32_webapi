use actix_web::web;
use crate::handlers::{data};

pub fn config_app(cfg: &mut web::ServiceConfig){
    cfg.service(
        web::scope("/datas")
        .service(
            web::resource("")
            .route(web::get().to(data::get_datas))
            .route(web::post().to(data::add_data))
            .route(web::put().to(data::update_data)), 
        )
        .service(
            web::scope("/{device}")
            .service(
                web::resource("")
                .route(web::get().to(data::get_data)),
            )
        ),
    ); 
}