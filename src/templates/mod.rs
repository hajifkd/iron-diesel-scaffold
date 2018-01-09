use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub title: &'a str,
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
