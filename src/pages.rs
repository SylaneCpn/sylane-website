#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::md::Md;
use crate::Navbar;


//##############################################################
//Md pages
#[component]
pub fn AboutFr() -> Element {
    rsx! {
    Navbar{}
    Md { content : include_str!("../md/about_fr.md")}
    }
}

#[component]
pub fn HomeFr() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/home_fr.md")}
    }
}


#[component]
pub fn ProjectsFr() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/projects_fr.md")}
    }
}

#[component]
pub fn ResumeFr() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/resume_fr.md")}
    }
}

#[component]
pub fn SchoolFr() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/school_fr.md")}
    }
}