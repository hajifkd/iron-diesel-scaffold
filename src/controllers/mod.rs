pub mod users;

use iron::prelude::*;
use iron::status;
use router::Router;

use templates;

pub fn router() -> Router {
    router!(
    index:      get   "/"               => index,
    index_q:    get   "/:name"          => index,
    insert:     post  "/insert/"        => users::insert,
    list_user:  get   "/list/"          => users::list
  )
}

fn index(req: &mut Request) -> IronResult<Response> {
    let ref name = req.extensions
        .get::<Router>()
        .unwrap()
        .find("name")
        .unwrap_or("hoge");
    Ok(Response::with((
        status::Ok,
        templates::IndexTemplate {
            _parent: templates::BaseTemplate { title: "index" },
            name: name,
        },
    )))
}
