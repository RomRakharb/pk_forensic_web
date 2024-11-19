pub mod components {
    use maud::{html, Markup};

    struct Theme;
    impl Theme {
        fn bg() -> String {
            String::from("background-color: #a93b4b")
        }
        fn text() -> String {
            String::from("color: #a93b4b")
        }
    }

    pub fn header() -> Markup {
        html! {
            div ."bg-white-600 text-black p-3" {
                div ."max-w-7xl mx-auto flex justify-between items-end"{
                    img src = "./static/logo.png" href = "/";
                    nav {
                        ul style={(Theme::text())} ."flex space-x-10"{
                            a href="#" ."hover:text-gray-300" {"หน้าแรก"}
                            a href="#" ."hover:text-gray-300" {"เกี่ยวกับเรา"}
                            a href="#" ."hover:text-gray-300" {"ติดต่อ"}
                            a href="#" ."hover:text-gray-300" {"ติดต่อ"}
                            a href="#" ."hover:text-gray-300" {"ติดต่อ"}
                            a href="#" ."hover:text-gray-300" {"ติดต่อ"}
                        }
                    }
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

    fn news() -> Markup {
        html! {}
    }

    pub fn footer() -> Markup {
        html! {
            footer style={(Theme::bg())} ."p-10" { "a" }
        }
    }
}
