#![allow(non_snake_case)]

use dioxus::prelude::*;
use crate::md::Md;
use crate::navbar::Navbar;
use crate::contacts::Contacts;
use crate::boxs::Box;


//##############################################################
//Md pages
#[component]
pub fn About() -> Element {
    rsx! {
    Navbar{}
    Md { content : include_str!("../md/about_fr.md")}
    Contacts {}
    }
}

#[component]
pub fn Home() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/home_fr.md")}
        Contacts {}
    }
}


#[component]
pub fn Projects() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/projects_fr.md")}
        Contacts {}
    }
}

#[component]
pub fn Resume() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/resume_fr.md")}
        Contacts {}
    }
}

#[component]
pub fn School() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/school_fr.md")}
        Box {title : "An interesting Title" , children : rsx! {Md {content : include_str!("../md/school_fr/test.md")}}}
        Contacts {}

    }
}

#[component]
pub fn Blog() -> Element {
    rsx! {
        Navbar{}
        Md { content : include_str!("../md/blog_fr.md")}
        Contacts {}
    }
}