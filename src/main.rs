#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate iron_diesel_middleware;
extern crate mount;
extern crate params;
#[macro_use]
extern crate router;
extern crate staticfile;
extern crate iron_sessionstorage;
extern crate iron_csrf_middleware;

use std::path::Path;
use iron::prelude::*;
use dotenv::dotenv;
use iron_diesel_middleware::DieselMiddleware;
use mount::Mount;
use staticfile::Static;
use iron_sessionstorage::SessionStorage;
use iron_sessionstorage::backends::SignedCookieBackend;
use iron_csrf_middleware::CsrfMiddleware;

#[macro_use]
extern crate askama;

mod db;
mod controllers;
mod templates;

fn main() {
    let csrf_middleware = CsrfMiddleware::new("hogehoge");
    let mut router = Chain::new(controllers::router());
    router.link_before(csrf_middleware);

    let mut mount = Mount::new();
    mount.mount("/", router);
    mount.mount("/static", Static::new(Path::new("static")));

    dotenv().ok();
    let database = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let diesel_middleware: DieselMiddleware<db::SqlConnection> =
        DieselMiddleware::new(&database).unwrap();
    let my_secret = b"fugafuga".to_vec();

    let mut chain = Chain::new(mount);
    chain.link_before(diesel_middleware);
    chain.link_around(SessionStorage::new(SignedCookieBackend::new(my_secret)));

    Iron::new(chain).http("localhost:3000").unwrap();
}
