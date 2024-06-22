use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn Links() -> Element {

    rsx! {
        Link {to : Route::HomeFr{} , "Acceuil"}
        Link {to : Route::ProjectsFr{} , "Projets"}
        Link {to : Route::ResumeFr{} , "CV"}
        Link {to : Route::AboutFr{} , "A propos"}
        Link {to : Route::SchoolFr{} , "Ecole"}
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