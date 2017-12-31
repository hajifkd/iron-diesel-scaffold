extern crate iron;
#[macro_use]
extern crate router;
#[macro_use]
extern crate diesel;
extern crate iron_diesel_middleware;
extern crate dotenv;

use iron::prelude::*;
use iron::status;
use router::Router;
use dotenv::dotenv;
use diesel::prelude::*;
use iron_diesel_middleware::{DieselMiddleware, DieselPooledConnection, DieselReqExt};

mod db;

fn main() {
    let router = router!(index: get "/" => handler,
                         query: get "/:query" => handler);

    dotenv().ok();
    let database = std::env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let diesel_middleware: DieselMiddleware<diesel::SqliteConnection> = DieselMiddleware::new(&database).unwrap();

    let mut chain = Chain::new(router);
    chain.link_before(diesel_middleware);

    Iron::new(chain).http("localhost:3000").unwrap();

    fn handler(req: &mut Request) -> IronResult<Response> {
        let con: DieselPooledConnection<diesel::SqliteConnection> = req.db_conn();

        use db::schema::users::dsl::*;
        use db::models::User;

        let results = users.load::<User>(&*con).expect("Error DB");

        for user in results {
          println!("{}", user.id);
        }
        
        let ref query = req.extensions.get::<Router>().unwrap().find("query").unwrap_or("/");
        Ok(Response::with((status::Ok, *query)))
    }
}