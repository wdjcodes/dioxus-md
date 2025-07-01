use dioxus::prelude::*;
use dioxus_md::TextField;

fn app() -> Element {
    rsx!(
        TextField { label: "Hello world!" }
    )
}

fn main() {
    dioxus::launch(app)
}