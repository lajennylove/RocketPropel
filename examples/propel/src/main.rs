#[macro_use]
extern crate rocket;

mod views;

use propel_rocket::PropelView;
use views::{Post, PostsIndex};

// The handler is the view composer: it builds the full context for the view.
#[get("/posts")]
fn posts() -> PropelView {
    let context = PostsIndex {
        title: "Posts".into(),
        posts: vec![
            Post {
                title: "Hello, Propel".into(),
                published_at: Some("2026-11-05".into()),
            },
            Post {
                title: "A draft".into(),
                published_at: None,
            },
        ],
    };

    PropelView::render("posts.index", &context)
}

#[launch]
fn rocket() -> _ {
    rocket::build().mount("/", routes![posts])
}
