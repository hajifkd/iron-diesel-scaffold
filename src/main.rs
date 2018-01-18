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

use std::path::Path;
use iron::prelude::*;
use dotenv::dotenv;
use iron_diesel_middleware::DieselMiddleware;
use mount::Mount;
use staticfile::Static;

#[macro_use]
extern crate askama;

mod db;
mod controllers;
mod templates;

fn main() {
    let mut mount = Mount::new();
    mount.mount("/", controllers::router());
    mount.mount("/static", Static::new(Path::new("static")));

    dotenv().ok();
    let database = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let diesel_middleware: DieselMiddleware<db::SqlConnection> =
        DieselMiddleware::new(&database).unwrap();

    let mut chain = Chain::new(mount);
    chain.link_before(diesel_middleware);

    Iron::new(chain).http("localhost:3000").unwrap();
}
