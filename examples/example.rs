use dioxus::prelude::*;
use dioxus_md::TextField;

fn app() -> Element {

    let value = use_signal(||"".to_string());
    
    rsx!(
        div {
            class: "example", 
            width: "96vw",
            height: "96vh",
            background: "gray",
            div { height: "56px" }
            TextField { label: "Hello world!", value: value }
         }
    )
}

fn main() {
    dioxus::launch(app)
}