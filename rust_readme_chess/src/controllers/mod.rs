use actix_web::web;

mod controller;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(controller::play)
       .service(controller::select)
       .service(controller::new_game);
}
