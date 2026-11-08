//! Typed contexts for the views in `views/`.
//!
//! Each view has one context struct here, and handlers build it. Today a
//! template that names a field the context lacks fails at render time. Once
//! views compile, the same mistake fails at `cargo check`.

use serde::Serialize;

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
}

#[derive(Debug, Serialize)]
pub struct PostShow {
    pub post: Post,
}

#[derive(Debug, Serialize)]
pub struct PageLink {
    pub href: String,
    pub label: String,
    pub covers: String,
}

#[derive(Debug, Serialize)]
pub struct Home {
    pub heading: String,
    pub intro: String,
    pub pages: Vec<PageLink>,
}

#[derive(Debug, Serialize)]
pub struct Escaping {
    pub dangerous: String,
    pub trusted: String,
    pub safe_url: String,
    pub unsafe_url: String,
    pub tab_url: String,
}

#[derive(Debug, Serialize)]
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
}

#[derive(Debug, Serialize)]
pub struct Dump {
    pub post: Post,
    pub tags: Vec<String>,
}

#[derive(Debug, Serialize)]
pub struct Cache {
    pub value: String,
}

#[derive(Debug, Serialize)]
pub struct Flag {
    pub flag: bool,
}

/// A context with no fields, for views that show a fixed message.
#[derive(Debug, Serialize)]
pub struct NoFields {}
