#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::md::Markdown;
use crate::Route;

//##############################################################
//Md pages
#[component]
pub fn AboutFr() -> Element {
    let class = use_signal(|| String::from("md_class"));
    rsx! {
        link {
            rel: "stylesheet",
            href: "/main.css"
        }
        Link { to: Route::Home {}, "Go to Home" }
        div { class: "md", Markdown { class: class, content: include_str!("../md/about_fr.md") } }
    }
}

#[component]
pub fn HomeFr() -> Element {
    let class = use_signal(|| String::from("md_class"));
    rsx! {
        link {
            rel: "stylesheet",
            href: "/main.css"
        }
        Link { to: Route::Home {}, "Go to Home" }
        div { class: "md", Markdown { class: class, content: include_str!("../md/home_fr.md") } }
    }
}


#[component]
pub fn ProjectsFr() -> Element {
    let class = use_signal(|| String::from("md_class"));
    rsx! {
        link {
            rel: "stylesheet",
            href: "/main.css"
        }
        Link { to: Route::Home {}, "Go to Home" }
        div { class: "md", Markdown { class: class, content: include_str!("../md/projects_fr.md") } }
    }
}

#[component]
pub fn ResumeFr() -> Element {
    let class = use_signal(|| String::from("md_class"));
    rsx! {
        link {
            rel: "stylesheet",
            href: "/main.css"
        }
        Link { to: Route::Home {}, "Go to Home" }
        div { class: "md", Markdown { class: class, content: include_str!("../md/resume_fr.md") } }
    }
}

#[component]
pub fn SchoolFr() -> Element {
    let class = use_signal(|| String::from("md_class"));
    rsx! {
        link {
            rel: "stylesheet",
            href: "/main.css"
        }
        Link { to: Route::Home {}, "Go to Home" }
        div { class: "md", Markdown { class: class, content: include_str!("../md/school_fr.md") } }
    }
}