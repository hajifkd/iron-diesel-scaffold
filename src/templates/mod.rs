use askama::Template;
use iron_csrf_middleware::CsrfReqExt;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub title: &'a str,
    pub csrf_token: String,
    pub query_key: &'static str,
}

impl<'a> BaseTemplate<'a> {
    pub fn new(req: &mut ::iron::Request, title: &'a str) -> BaseTemplate<'a> {
        BaseTemplate {
            title: title,
            csrf_token: req.csrf_token(),
            query_key: ::iron_csrf_middleware::QUERY_KEY,
        }
    }
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a> {
    pub _parent: BaseTemplate<'a>,
    pub name: &'a str,
}

#[derive(Template)]
#[template(path = "list_user.html")]
pub struct ListUserTemplate<'a> {
    pub _parent: BaseTemplate<'a>,
    pub users: &'a [::db::models::User],
}
