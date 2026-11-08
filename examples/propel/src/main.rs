#[macro_use]
extern crate rocket;

mod nav;
mod views;

use nav::Page;


use propel_rocket::PropelView;
use rocket::fs::FileServer;
use rocket::http::Status;
use views::*;

/// Sample data. A real app would load this from a database.
fn all_posts() -> Vec<Post> {
    vec![
        Post {
            id: 1,
            title: "Hello, Propel".into(),
            published_at: Some("2026-11-05".into()),
            body: "The first post, rendered by the dev renderer.".into(),
        },
        Post {
            id: 2,
            title: "A draft".into(),
            published_at: None,
            body: "Drafts have no date, so the @if takes its else branch.".into(),
        },
        Post {
            id: 3,
            title: "Escaping <b>works</b>".into(),
            published_at: Some("2026-11-06".into()),
            body: "Titles are escaped like every other text echo.".into(),
        },
    ]
}

#[get("/")]
fn home() -> PropelView {
    PropelView::render(
        "home",
        &Page::new(
            "/",
            Article {
                title: "Introduction".into(),
            },
        ),
    )
}

/// The documentation pages. The address must be in the sidebar, so only the
/// pages the navigation lists are served.
#[get("/docs/<page>")]
fn docs(page: &str) -> Result<PropelView, (Status, PropelView)> {
    let path = format!("/docs/{page}");

    match nav::title(&path) {
        Some(title) => Ok(PropelView::render(
            &format!("docs.{page}"),
            &Page::new(&path, Article { title }),
        )),
        None => Err(not_found_page()),
    }
}

#[get("/posts")]
fn posts() -> PropelView {
    PropelView::render(
        "posts.index",
        &Page::new(
            "/posts",
            PostsIndex {
                title: "Posts".into(),
                posts: all_posts(),
            },
        ),
    )
}

/// The id is read as text, so a value that is not a number gets the same 404
/// page as an unknown one, not Rocket's own 422.
#[get("/posts/<id>")]
fn post(id: &str) -> Result<PropelView, (Status, PropelView)> {
    let found = id
        .parse::<u32>()
        .ok()
        .and_then(|id| all_posts().into_iter().find(|post| post.id == id));

    match found {
        Some(post) => Ok(PropelView::render(
            "posts.show",
            &Page::new(&format!("/posts/{id}"), PostShow { post }),
        )),
        None => Err(not_found_page()),
    }
}

#[get("/features/escaping")]
fn escaping() -> PropelView {
    PropelView::render(
        "features.escaping",
        &Page::new("/features/escaping", Escaping {
            dangerous: "<script>alert('x')</script>".into(),
            trusted: "<em>trusted</em>".into(),
            safe_url: "https://example.com/?a=1&b=2".into(),
            unsafe_url: "javascript:alert('x')".into(),
            tab_url: "java\tscript:alert(1)".into(),
        }),
    )
}

#[get("/features/control")]
fn control() -> PropelView {
    PropelView::render(
        "features.control",
        &Page::new("/features/control", Control {
            is_admin: false,
            is_member: true,
            tags: vec!["rust".into(), "rocket".into(), "propel".into()],
            empty_items: vec![],
            author: None,
            maybe_items: None,
        }),
    )
}

#[get("/features/dump")]
fn dump() -> PropelView {
    PropelView::render(
        "features.dump",
        &Page::new(
            "/features/dump",
            Dump {
                post: all_posts().remove(0),
                tags: vec!["rust".into(), "rocket".into()],
            },
        ),
    )
}

#[get("/features/dd")]
fn dd() -> PropelView {
    PropelView::render(
        "features.dd",
        &Page::new(
            "/features/dd",
            Dump {
                post: all_posts().remove(0),
                tags: vec!["rust".into(), "rocket".into()],
            },
        ),
    )
}

#[get("/features/cache")]
fn cache() -> PropelView {
    PropelView::render(
        "features.cache",
        &Page::new("/features/cache", Cache { value: "v1".into() }),
    )
}

#[get("/errors/missing")]
fn missing() -> PropelView {
    // The view names `$not_in_context`, which this context does not have.
    PropelView::render(
        "errors.missing",
        &Page::new(
            "/errors/missing",
            Article {
                title: "Missing variable".into(),
            },
        ),
    )
}

#[get("/errors/nested-section")]
fn nested_section() -> PropelView {
    PropelView::render(
        "errors.nested_section",
        &Page::new(
            "/errors/nested-section",
            Article {
                title: "Nested section".into(),
            },
        ),
    )
}

fn not_found_page() -> (Status, PropelView) {
    (
        Status::NotFound,
        PropelView::render("errors.not_found", &Page::new("", NoFields {})),
    )
}

#[catch(404)]
fn not_found() -> (Status, PropelView) {
    not_found_page()
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount(
            "/",
            routes![
                home,
                docs,
                posts,
                post,
                escaping,
                control,
                dump,
                dd,
                cache,
                missing,
                nested_section
            ],
        )
        // Built assets: hashed files and copied images and fonts.
        .mount("/build", FileServer::new("public/build"))
        .register("/", catchers![not_found])
}
