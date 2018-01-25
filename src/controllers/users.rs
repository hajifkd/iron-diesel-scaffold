use iron::prelude::*;
use iron::status;
use iron::modifiers::Redirect;

use diesel;
use diesel::prelude::*;
use iron_diesel_middleware::{DieselPooledConnection, DieselReqExt};
use params::{Params, Value};
use db;
use templates::BaseTemplate;

use templates;

pub fn list(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::SqlConnection> = req.db_conn();

    use db::schema::users::dsl::*;
    use db::models::User;

    let results = users.load::<User>(&*con).expect("Error reading DB");

    Ok(Response::with((
        status::Ok,
        templates::ListUserTemplate {
            _parent: BaseTemplate::new(req, "list user"),
            users: &results,
        },
    )))
}

pub fn insert(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<db::SqlConnection> = req.db_conn();

    use db::schema::users;
    use db::models::NewUser;

    if let (Some(Value::String(name)), Some(Value::String(email))) = {
        // However, considering the possibility that some of the middleware uses
        // Params plugin, using the reference may be better.
        // To be honest, `compute` is actually possible
        // unless large files are treated or something like that.
        let params = req.get_ref::<Params>().unwrap();
        (
            params.get("name").map(Value::clone),
            params.get("email").map(Value::clone),
        )
    } {
        let new_user = NewUser {
            name: &name,
            email: &email,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&*con)
            .expect("INSERT failed");

        Ok(Response::with((
            status::Found,
            Redirect(url_for!(req, "list_user")),
        )))
    } else {
        Ok(Response::with(status::NotFound))
    }
}
