//! Compiles the views in `resources/views` into the binary, with the context
//! type each view is rendered with. The generated module is included in
//! `main.rs`. See `propel_compiler::build::Views` and decision 0004.
//!
//! `errors.missing` and `errors.nested_section` are not listed. They are
//! debug-only demos of the dev renderer, and they would stop this build if
//! they were compiled. Their templates are still parsed, so a syntax error
//! in them stops the build.

fn main() {
    let view = |context: &str| format!("crate::views::{context}");

    propel_compiler::build::Views::new("resources/views")
        .view("home", view("Article"))
        .view("docs.assets", view("Article"))
        .view("docs.components", view("Article"))
        .view("docs.context", view("Article"))
        .view("docs.install", view("Article"))
        .view("docs.layouts", view("Article"))
        .view("docs.syntax", view("Article"))
        .view("docs.vite-tailwind", view("Article"))
        .view("posts.index", view("PostsIndex"))
        .view("posts.show", view("PostShow"))
        .view("features.escaping", view("Escaping"))
        .view("features.control", view("Control"))
        .view("features.dump", view("Dump"))
        .view("features.dd", view("Dump"))
        .view("features.cache", view("Cache"))
        .view("errors.not_found", view("NoFields"))
        .build();
}
