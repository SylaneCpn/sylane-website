use dioxus::prelude::*;

pub fn Contacts() -> Element {
    rsx! {
        div { class : "contact_bar",
            h2 { "Me contacter"}
            div { class : "contact_elements",

                div { class : "c_item",
                    div{ class : "header",
                        p {"Email"}
                        img{ src : "sylane-website/img/pics/mail.svg"} 
                    }
                    a {href  : "" , "s0campan@enib.fr"}
                }

                div { class : "c_item",
                    div{ class : "header",
                        p {"LinkedIn"}
                        img{ src : "sylane-website/img/pics/linked_in.svg"} 
                    }
                    a { href : "", "sylane-campan"}
                }

                div { class : "c_item",
                    div{ class : "header",
                        p {"Github"}
                        img{ src : "sylane-website/img/pics/github.svg"} 
                    }
                    a { href : "", "SylaneCpn"}
                }

            }

        }
    }
}