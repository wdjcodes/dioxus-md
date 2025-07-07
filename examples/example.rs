use dioxus::prelude::*;
use dioxus_md::TextField;

fn app() -> Element {
    rsx!(
        div { 
            width: "96vw",
            height: "96vh",
            background: "black",
            TextField { label: "Hello world!" }
         }
    )
}

fn main() {
    dioxus::launch(app)
}