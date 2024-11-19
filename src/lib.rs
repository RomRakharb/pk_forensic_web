pub mod components {
    use maud::{html, Markup};

    pub fn header() -> Markup {
        html! {
            div ."bg-white-600 text-black p-5" {
                div ."max-w-7xl mx-auto flex justify-between items-center"{
                    img src = "./static/logo.png" href = "/";
                    nav {
                        ul ."flex space-x-6"{
                            a href="#" ."hover:text-gray-300" {"หน้าแรก"}
                            a href="#" ."hover:text-gray-300" {"เกี่ยวกับเรา"}
                            a href="#" ."hover:text-gray-300" {"ติดต่อ"}
                        }
                    }
                }
            }
        }
    }

    pub fn nav_bar() -> Markup {
        html! {
            nav ."bg-red-600 py-4" {
                div ."flex justify-center space-x-6" {
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                    button ."text-white px-4 py-2 hover:bg-red-500" { "test" }
                }
            }
        }
    }

    pub fn carousel() -> Markup {
        html! {
            div ."bg-gray-300 py-10" {

            }
        }
    }
}
