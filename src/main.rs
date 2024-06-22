#![allow(non_snake_case)]
mod md;
mod pages;
mod navbar;

use dioxus::prelude::*;
use tracing::Level;
use pages::*;
use navbar::Navbar;

#[derive(Clone, Routable, Debug, PartialEq)]
enum Route {
    #[route("/")]
    HomeFr {},
    #[route("/about_fr")]
    AboutFr {},
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



