pub mod pages {
    use crate::components::{footer, head, header};
    use maud::{html, Markup};

    pub async fn home() -> Markup {
        html! {
            (head("พิสูจน์หลักฐานจังหวัดภูเก็ต"))
            body ."bg-gray-200" {
                (header())
                hr ."p-1 bg-[#aa3b4b]" ;
                h1 { "Hello, World!" }
                (footer())
            }
        }
    }

    pub async fn news() -> Markup {
        html! {
            (head("พิสูจน์หลักฐานจังหวัดภูเก็ต"))
            body  {
                (header())
                h1 { "Hello, World!" }
                (footer())
            }
        }
    }
}

mod components {
    use maud::{html, Markup, DOCTYPE};

    pub fn head(title: &str) -> Markup {
        html! {
            head {
                (DOCTYPE)
                meta charset="utf-8";
                title { (title) }
                script src="https://cdn.tailwindcss.com" {}
            }
        }
    }

    pub fn header() -> Markup {
        html! {
            div ."overflow-hidden bg-white py-5 px-2.5 flex items-center" {
                img ."h-20" src="./static/logo.png";
                div ."ml-auto" {
                    a ."text-white text-center p-3 no-underline text-lg leading-6 rounded bg-[#aa3b4b]" href="#contact" { "Contact" }
                    a ."text-black text-center p-3 no-underline text-lg leading-6 rounded" href="#contact" { "Contact" }
                    a ."text-black text-center p-3 no-underline text-lg leading-6 rounded" href="#contact" { "Contact" }
                }
            }
        }
    }

    pub fn footer() -> Markup {
        html! {}
    }
}
