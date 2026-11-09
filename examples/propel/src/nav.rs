//! The site's navigation: the sidebar groups, and the previous and next page
//! for each page. Both come from one ordered list, so the sidebar and the pager
//! always agree.

use propel_macros::Truthy;
use serde::Serialize;

#[derive(Debug, Clone, Serialize, Truthy)]
pub struct Link {
    pub href: String,
    pub label: String,
}

#[derive(Debug, Serialize)]
pub struct Group {
    pub heading: String,
    pub links: Vec<Link>,
}

#[derive(Debug, Serialize)]
pub struct Pager {
    pub prev: Option<Link>,
    pub next: Option<Link>,
}

/// Every page in the sidebar, in order. Previous and next follow this order.
/// The groups are the sections of the site: introduction, installation,
/// Vite and Tailwind, Blade syntax, assets, and examples.
fn groups() -> Vec<Group> {
    let group = |heading: &str, links: &[(&str, &str)]| Group {
        heading: heading.into(),
        links: links
            .iter()
            .map(|(href, label)| Link {
                href: (*href).into(),
                label: (*label).into(),
            })
            .collect(),
    };

    vec![
        group("Introduction", &[("/", "Introduction")]),
        group(
            "Installation",
            &[("/docs/install", "Installation and configuration")],
        ),
        group(
            "Vite and Tailwind",
            &[
                ("/docs/vite-tailwind", "Setup and hot reload"),
                ("/features/cache", "Template cache"),
            ],
        ),
        group(
            "Blade syntax",
            &[
                ("/docs/syntax", "Syntax overview"),
                ("/docs/layouts", "Layouts and sections"),
                ("/docs/context", "Context and composers"),
                ("/docs/components", "Components and reuse"),
                ("/features/escaping", "Escaping"),
                ("/features/control", "Control flow"),
                ("/features/dump", "@dump"),
                ("/features/dd", "@dd"),
            ],
        ),
        group("Assets", &[("/docs/assets", "Images, fonts and assets")]),
        group(
            "Examples",
            &[
                ("/posts", "All posts"),
                ("/posts/1", "One post"),
                ("/errors/missing", "Missing variable"),
                ("/errors/nested-section", "Nested section"),
            ],
        ),
    ]
}

/// The sidebar title of the page at `path`, if the page is in the sidebar.
pub fn title(path: &str) -> Option<String> {
    groups()
        .into_iter()
        .flat_map(|group| group.links)
        .find(|link| link.href == path)
        .map(|link| link.label)
}

/// The paths for the sitemap: the home page and the documentation pages, in
/// sidebar order. The demo pages (posts, features and errors) are left out.
pub fn sitemap_paths() -> Vec<String> {
    groups()
        .into_iter()
        .flat_map(|group| group.links)
        .map(|link| link.href)
        .filter(|href| href == "/" || href.starts_with("/docs/"))
        .collect()
}

/// The navigation every page shares. Each view's context has `nav` and `pager`
/// fields with these values. They are fields of the context itself, not of a
/// nested struct, because compiled views read the Rust fields by name.
pub struct Chrome {
    pub nav: Vec<Group>,
    pub pager: Pager,
}

impl Chrome {
    /// `path` is the page's address. Only pages in the sidebar get a previous
    /// and a next link.
    pub fn at(path: &str) -> Self {
        let nav = groups();
        let links: Vec<Link> = nav.iter().flat_map(|group| group.links.clone()).collect();
        let index = links.iter().position(|link| link.href == path);

        let pager = Pager {
            prev: index
                .and_then(|i| i.checked_sub(1))
                .map(|i| links[i].clone()),
            next: index.and_then(|i| links.get(i + 1).cloned()),
        };

        Chrome { nav, pager }
    }
}

#[cfg(test)]
mod tests {
    use crate::render_docs;
    use crate::views::Article;

    use super::*;

    /// Every documentation page in the sidebar must have a compiled view, or
    /// its link is a 404.
    #[test]
    fn every_docs_link_in_the_sidebar_has_a_compiled_view() {
        let links: Vec<Link> = groups().into_iter().flat_map(|group| group.links).collect();
        let docs: Vec<&Link> = links
            .iter()
            .filter(|link| link.href.starts_with("/docs/"))
            .collect();
        assert!(!docs.is_empty());

        for link in docs {
            let page = link.href.trim_start_matches("/docs/");
            let context = Article {
                title: link.label.clone(),
                nav: Vec::new(),
                pager: Pager {
                    prev: None,
                    next: None,
                },
            };
            assert!(
                render_docs(page, &context).is_some(),
                "no compiled view for {}",
                link.href
            );
        }
    }
}
