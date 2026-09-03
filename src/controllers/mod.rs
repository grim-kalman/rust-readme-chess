use actix_web::web;

mod controller;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/health").route(web::get().to(crate::controllers::controller::health)))
        .service(web::resource("/play").route(web::get().to(crate::controllers::controller::play)))
        .service(
            web::resource("/select").route(web::get().to(crate::controllers::controller::select)),
        )
        .service(
            web::resource("/new").route(web::get().to(crate::controllers::controller::new_game)),
        )
        .service(
            web::resource("/robots.txt").route(web::get().to(crate::controllers::controller::robots)),
        );
}
