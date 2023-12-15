// src/lib/configuration.rs

// dependencies
use actix_web::web::{self, ServiceConfig};
use minus1::{error, root};
use shuttle_actix_web::ShuttleActixWeb;

// configuration
pub fn configure_services(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("").service(root).service(error));
}

// startup
pub fn startup_service() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static>
{
    let config = move |cfg: &mut ServiceConfig| {
        configure_services(cfg);
    };

    Ok(config.into())
}
