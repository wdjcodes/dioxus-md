use dioxus::prelude::*;
use dioxus_md::TextField;

fn app() -> Element {
    
    rsx!(
        div {
            class: "example", 
            width: "96vw",
            height: "96vh",
            background: "gray",
            TextField { label: "Hello world!" }
         }
    )
}

fn main() {
    dioxus::launch(app)
}