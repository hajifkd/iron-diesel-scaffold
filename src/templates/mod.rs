use askama::Template;

#[derive(Template)]
#[template(path = "base.html")]
pub struct BaseTemplate<'a> {
    pub title: &'a str,
}

#[derive(Template)]
#[template(path = "index.html")]
pub struct IndexTemplate<'a, 'b> {
    pub _parent: BaseTemplate<'a>,
    pub name: &'b str,
}

#[derive(Template)]
#[template(path = "list_user.html")]
pub struct ListUserTemplate<'a, 'b> {
    pub _parent: BaseTemplate<'a>,
    pub users: &'b [::db::models::User],
}
