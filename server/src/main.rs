mod init;

use actix_identity::{CookieIdentityPolicy, IdentityService};
use actix_web::middleware::Logger;
use actix_web::{web, App, FromRequest, HttpRequest, HttpServer, Responder, Result};
use env_logger::Env;

use crate::init::Init;
use actix_files::{Files, NamedFile};
use arangors::Connection;
use std::sync::Arc;

mod auth;
mod models;

mod utils;

async fn index() -> Result<NamedFile> {
    Ok(NamedFile::open("../client/index.html")?)
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let init = Init::new();
    let builder = init.build_ssl();
    let domain = init.domain().to_string();

    let conn = init.connect_db().await;
    let conn = Arc::new(conn);

    let server = HttpServer::new(move || {
        App::new()
            .data(conn.clone())
            .wrap(Logger::new("%r %s %D ms %a"))
            .wrap(IdentityService::new(
                CookieIdentityPolicy::new(utils::password::SECRET_KEY.as_bytes())
                    .name("auth")
                    .path("/")
                    .domain(domain.as_str())
                    .max_age_time(chrono::Duration::days(1))
                    .secure(false), // this can only be true if you have https todo put to true in production
            ))
            .data(web::JsonConfig::default().limit(4096))
            .service(
                web::scope("/api")
                    // todo make invitation system later
                    // .service(
                    //     web::resource("/register/{invitation_id}")
                    //         .route(web::post().to(register_handler::register_user)),
                    // )
                    // .service(
                    //     web::resource("/register").route(web::post().to(register::register_user)),
                    // )
                    .default_service(web::route().to(web::HttpResponse::NotFound)),
            )
            .service(Files::new("/pkg", "../client/pkg"))
            .default_service(web::get().to(index))
    })
    .workers(*init.workers());

    server.bind_openssl("127.0.0.1:8000", builder)?.run().await
}
