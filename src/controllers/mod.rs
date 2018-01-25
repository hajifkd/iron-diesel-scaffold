pub mod users;

use iron::prelude::*;
use iron::status;
use mount::Mount;
use router::Router;

use templates::{self, BaseTemplate};

pub fn router() -> Mount {
    let router = router!(
        index:      get   "/"               => index,
        index_q:    get   "/:name"          => index,
        insert:     post  "/insert/"        => users::insert,
        list_user:  get   "/list/"          => users::list
    );

    let mut mount = Mount::new();
    mount.mount("/", router);

    mount
}

fn index(req: &mut Request) -> IronResult<Response> {
    let ref name = req.extensions
        .get::<Router>()
        .unwrap()
        .find("name")
        .unwrap_or("hoge")
        .to_owned();
    Ok(Response::with((
        status::Ok,
        templates::IndexTemplate {
            _parent: BaseTemplate::new(req, "index"),
            name: &name,
        },
    )))
}
