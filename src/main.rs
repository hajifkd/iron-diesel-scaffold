#[macro_use]
extern crate diesel;
extern crate dotenv;
extern crate iron;
extern crate iron_diesel_middleware;
extern crate params;
#[macro_use]
extern crate router;

use iron::prelude::*;
use dotenv::dotenv;
use iron_diesel_middleware::DieselMiddleware;

#[macro_use]
extern crate askama;

mod db;
mod controllers;
mod templates;

fn main() {
    let router = controllers::router();

    dotenv().ok();
    let database = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let diesel_middleware: DieselMiddleware<diesel::SqliteConnection> =
        DieselMiddleware::new(&database).unwrap();

    let mut chain = Chain::new(router);
    chain.link_before(diesel_middleware);

    Iron::new(chain).http("localhost:3000").unwrap();
}
