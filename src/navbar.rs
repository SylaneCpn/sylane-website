use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn Links() -> Element {

    rsx! {
        Link {to : Route::Home {}, "Home"}
        Link {to : Route::HomeFr{} , "Home fr"}
        Link {to : Route::ProjectsFr{} , "Projects"}
        Link {to : Route::ResumeFr{} , "Resume"}
        Link {to : Route::AboutFr{} , "About"}
        Link {to : Route::SchoolFr{} , "School"}
    }
                    
}


#[component]
pub fn Navbar() -> Element {

    let mut extended = use_signal(|| false);
    rsx! {
        link { rel: "stylesheet", href: "/navbar.css" }
        div {class : "extended_nav",
            nav { class : "navbar",
                div { class : "links" ,
                    Links {}
                }
                img {class : "svg" , src : "/hamburger.svg" , onclick : move |_| extended.toggle()}
            
            
            }

            if extended() {
                div { class : "extended" ,
                    Links {}
                }
            }
        }
    }
}