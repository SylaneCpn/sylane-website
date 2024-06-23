use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn Links() -> Element {

    rsx! {
        Link {to : Route::HomeFr{} , "Acceuil"}
        Link {to : Route::ResumeFr{} , "CV"}
        Link {to : Route::SchoolFr{} , "Ecole"}
        Link {to : Route::ProjectsFr{} , "Projets"}
        Link {to : Route::AboutFr{} , "A propos"}
        
    }
                    
}


#[component]
pub fn Navbar() -> Element {

    let mut extended = use_signal(|| false);
    rsx! {
        link { rel: "stylesheet", href: "sylane-website/navbar.css" }
        div {class : "extended_nav",
            nav { class : "navbar",
                div { class : "links" ,
                    Links {}
                }
                img {class : "svg" , src : "sylane-website/hamburger.svg" , onclick : move |_| extended.toggle()}
                
            
            
            }

            if extended() {
                div { class : "extended" ,
                    Links {}
                }
            }
        }
    }
}