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
    let link = |href: &str, label: &str, covers: &str| PageLink {
        href: href.into(),
        label: label.into(),
        covers: covers.into(),
    };

    PropelView::render(
        "home",
        &Page::new("/", Home {
            heading: "Propel test pages".into(),
            intro: "Each page exercises one part of Propel. Follow the links.".into(),
            pages: vec![
                link("/posts", "Posts", "routes, @foreach, @forelse, layouts, @extends"),
                link("/posts/1", "One post", "route parameters, @if with @else, sections"),
                link("/features/escaping", "Escaping", "{{ }}, {!! !!}, URL checks, @@ and @{{"),
                link("/features/control", "Control flow", "@if / @elseif / @else, @forelse / @empty, null values"),
                link("/features/dump", "@dump", "debug helper: prints and keeps rendering"),
                link("/features/dd", "@dd", "debug helper: prints and stops the page"),
                link("/features/cache", "Template cache", "edit a view and refresh, no restart"),
                link("/errors/missing", "Error: missing variable", "500 in debug, generic in release"),
                link("/errors/nested-section", "Error: nested section", "a @section inside @if is rejected"),
            ],
        }),
    )
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
fn post(id: &str) -> (Status, PropelView) {
    let found = id
        .parse::<u32>()
        .ok()
        .and_then(|id| all_posts().into_iter().find(|post| post.id == id));

    match found {
        Some(post) => (
            Status::Ok,
            PropelView::render("posts.show", &Page::new(&format!("/posts/{id}"), PostShow { post })),
        ),
        None => not_found_page(),
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
    PropelView::render("errors.missing", &Page::new("/errors/missing", NoFields {}))
}

#[get("/errors/nested-section")]
fn nested_section() -> PropelView {
    PropelView::render(
        "errors.nested_section",
        &Page::new("/errors/nested-section", Flag { flag: true }),
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
