use iron::prelude::*;
use iron::status;
use iron::headers::ContentType;
use iron::modifiers::Redirect;

use diesel;
use diesel::prelude::*;
use iron_diesel_middleware::{DieselPooledConnection, DieselReqExt};
use params::{Params, Value};

use templates;

pub fn list(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<diesel::SqliteConnection> = req.db_conn();

    use db::schema::users::dsl::*;
    use db::models::User;

    let results = users.load::<User>(&*con).expect("Error reading DB");

    Ok(Response::with((
        ContentType::html().0,
        status::Ok,
        templates::ListUserTemplate {
            _parent: templates::BaseTemplate { title: "list user" },
            users: &results,
        },
    )))
}

pub fn insert(req: &mut Request) -> IronResult<Response> {
    let con: DieselPooledConnection<diesel::SqliteConnection> = req.db_conn();

    use db::schema::users;
    use db::models::NewUser;

    let url = url_for!(req, "list_user");

    let params = req.get_ref::<Params>().unwrap();

    if let (Some(&Value::String(ref name)), Some(&Value::String(ref email))) =
        (params.get("name"), params.get("email"))
    {
        let new_user = NewUser {
            name: name,
            email: email,
        };

        diesel::insert_into(users::table)
            .values(&new_user)
            .execute(&*con)
            .expect("INSERT failed");

        Ok(Response::with((status::Found, Redirect(url))))
    } else {
        Ok(Response::with(status::NotFound))
    }
}
