use dioxus::{logger::tracing, prelude::*};
use dioxus_md::TextField;

fn app() -> Element {

    let value = use_signal(||"Test".to_string());

    use_effect(move|| tracing::debug!("{}",value.read()));
    
    rsx!(
        div {
            class: "example", 
            width: "96vw",
            height: "96vh",
            background: "#141218",
            div { height: "56px" }
            TextField { label: "Enabled", value: value, disabled: false, }
            div { height: "56px" }
            TextField { label: "Disabled", disabled: true, }
         }
    )
}

fn main() {
    dioxus::launch(app)
}