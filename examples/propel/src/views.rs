//! Typed contexts for the views in `resources/views/`.
//!
//! Each view has one context struct here, and handlers build it. Compiled
//! views read the fields of these structs by name, so each one lists `nav`
//! and `pager` itself: a `#[serde(flatten)]` wrapper would work in the dev
//! renderer and not in a compiled view. The build (`build.rs`) checks every
//! field a template reads against these structs.

use propel_macros::Truthy;
use serde::Serialize;

use crate::nav::{Group, Pager};

#[derive(Debug, Clone, Serialize)]
pub struct Post {
    pub id: u32,
    pub title: String,
    pub published_at: Option<String>,
    pub body: String,
}

#[derive(Debug, Serialize)]
pub struct PostsIndex {
    pub title: String,
    pub posts: Vec<Post>,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

#[derive(Debug, Serialize)]
pub struct PostShow {
    pub post: Post,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

#[derive(Debug, Serialize)]
pub struct Escaping {
    pub dangerous: String,
    pub trusted: String,
    pub safe_url: String,
    pub unsafe_url: String,
    pub tab_url: String,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

#[derive(Debug, Serialize, Truthy)]
pub struct Author {
    pub name: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Control {
    pub is_admin: bool,
    pub is_member: bool,
    pub tags: Vec<String>,
    pub empty_items: Vec<String>,
    pub author: Option<Author>,
    pub maybe_items: Option<Vec<String>>,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

#[derive(Debug, Serialize)]
pub struct Dump {
    pub post: Post,
    pub tags: Vec<String>,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

#[derive(Debug, Serialize)]
pub struct Cache {
    pub value: String,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

/// A page with only a title, such as the documentation pages.
#[derive(Debug, Serialize)]
pub struct Article {
    pub title: String,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

/// A context with no fields of its own, for views that show a fixed message.
#[derive(Debug, Serialize)]
pub struct NoFields {
    pub nav: Vec<Group>,
    pub pager: Pager,
}
