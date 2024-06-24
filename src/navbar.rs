use dioxus::prelude::*;
use crate::Route;


#[component]
pub fn Links() -> Element {

    rsx! {
        Link {to : Route::Home{} , "Acceuil"}
        Link {to : Route::Resume{} , "CV"}
        Link {to : Route::School{} , "Ecole"}
        Link {to : Route::Projects{} , "Projets"}
        Link {to : Route::About{} , "A propos"}
        Link {to : Route::Blog{} , "Blog"}
        
    }
                    
}


#[component]
pub fn Navbar() -> Element {

    let mut extended = use_signal(|| false);
    rsx! {
        // link { rel: "stylesheet", href: "navbar.css" }
        div {class : "extended_nav",
            nav { class : "navbar",
                div { class : "links" ,
                    Links {}
                }
                img {class : "svg" , src : "img/pics/hamburger.svg" , onclick : move |_| extended.toggle()}
                
            
            
            }

            if extended() {
                div { class : "extended" ,
                    Links {}
                }
            }
        }
    }
}