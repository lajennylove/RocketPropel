//! Typed contexts for the views in `views/`.
//!
//! Each view has one context struct here. Handlers build it.
//!
//! Today, a template that names a field the context lacks fails at render
//! time, as an "undefined variable" error. The planned compiled views will
//! report the same mistake at `cargo check`. See `propel/README.md`, "Release
//! builds", for the current state.

use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct Post {
    pub title: String,
    pub published_at: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct PostsIndex {
    pub title: String,
    pub posts: Vec<Post>,
}
