## Home Fr

some text here 

```rust 

#![allow(non_snake_case)]
mod md;
mod md_pages;

use dioxus::prelude::*;
use tracing::Level;
use md_pages::*;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    Home {},
    #[route("/about_fr")]
    AboutFr {},
    #[route("/home_fr")]
    HomeFr {},
    #[route("/projects_fr")]
    ProjectsFr {},
    #[route("/resume_fr")]
    ResumeFr {},
    #[route("/school_fr")]
    SchoolFr {},
}

fn main() {
    // Init logger
    dioxus_logger::init(Level::INFO).expect("failed to init logger");
    launch(App);
}

fn App() -> Element {
    rsx! {
        link { rel: "stylesheet", href: "/main.css" }
        Router::<Route> {}
    }
}



#[component]
fn Home() -> Element {
    let mut count = use_signal(|| 0);

    rsx! {

        Link {
            to: Route::Home {
                
            },
            "Go to Home"
        }
        Link {
            to: Route::AboutFr {
                
            },
            "Go to About"
        }

        Link {
            to: Route::HomeFr {
                
            },
            "Go to Home Fr"
        }

        Link {
            to: Route::ProjectsFr {
                
            },
            "Go to Projects"
        }

        Link {
            to: Route::ResumeFr {
                
            },
            "Go to Resume"
        }


        Link {
            to: Route::SchoolFr {
                
            },
            "Go to School"
        }

        
        div {
            h1 { "High-Five counter: {count}" }
            button { onclick: move |_| count += 1, "Up high!" }
            button { onclick: move |_| count -= 1, "Down low!" }
            button { onclick: move |_| count -= 1, "Second Down low!" }
            div { "Hello Everyone !"}
        }
    }
}

```