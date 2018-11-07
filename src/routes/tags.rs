use rocket_contrib::templates::Template;
use serde_json;

use plume_models::{
    db_conn::DbConn,
    posts::Post,
    users::User,
};
use routes::{total_pages, page_limits};

#[get("/tag/<name>")]
pub fn tag(user: Option<User>, conn: DbConn, name: String) -> Template {
    paginated_tag(user, conn, name, 1)
}

#[get("/tag/<name>?<page>")]
pub fn paginated_tag(user: Option<User>, conn: DbConn, name: String, page: i32) -> Template {
    let posts = Post::list_by_tag(&*conn, name.clone(), page_limits(page));
    Template::render("tags/index", json!({
        "tag": name.clone(),
        "account": user.map(|u| u.to_json(&*conn)),
        "articles": posts.into_iter().map(|p| p.to_json(&*conn)).collect::<Vec<serde_json::Value>>(),
        "page": page,
        "n_pages": total_pages(Post::count_for_tag(&*conn, name) as i32)
    }))
}
