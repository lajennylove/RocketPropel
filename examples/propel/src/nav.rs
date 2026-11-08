//! The site's navigation: the sidebar groups, and the previous and next page
//! for each page. Both come from one ordered list, so the sidebar and the pager
//! always agree.

use serde::Serialize;

#[derive(Debug, Clone, Serialize)]
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

/// A page's own data, plus the navigation every layout needs.
#[derive(Debug, Serialize)]
pub struct Page<C> {
    #[serde(flatten)]
    pub content: C,
    pub nav: Vec<Group>,
    pub pager: Pager,
}

impl<C: Serialize> Page<C> {
    /// `path` is the page's address. Only pages in the sidebar get a previous
    /// and a next link.
    pub fn new(path: &str, content: C) -> Self {
        let nav = groups();
        let links: Vec<Link> = nav.iter().flat_map(|group| group.links.clone()).collect();
        let index = links.iter().position(|link| link.href == path);

        let pager = Pager {
            prev: index
                .and_then(|i| i.checked_sub(1))
                .map(|i| links[i].clone()),
            next: index.and_then(|i| links.get(i + 1).cloned()),
        };

        Page {
            content,
            nav,
            pager,
        }
    }
}
