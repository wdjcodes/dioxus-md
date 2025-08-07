use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use crate::{make_spring, TextFieldAttrs};

const STYLE_SHEET: Asset = asset!("/src/textfield/styles.css");
#[component]
pub fn TextField(
    props: TextFieldAttrs
) -> Element {
    let mut is_populated = use_signal(|| false);

    let focus_in = Transform{x: 12.0, y: 8.0, scale: 0.75, rotation: 0.0};
    let focus_out = Transform {x: 12.0, y: 16.0, scale: 1.0, rotation: 0.0};
    let mut transform = use_motion(focus_out);
    let mut focused = use_signal(||false);
    use_effect(move || {
        let ms = AnimationConfig::new(AnimationMode::Spring(make_spring(3800.0, 25.0, 1.0)));
        if(focused()){
            transform.animate_to(focus_in, ms);
        } else {
            transform.animate_to(focus_out, ms);
        }
    });

    rsx!(
        document::Stylesheet{ href: STYLE_SHEET }
        div {
            class: "textfield",
            tabindex: "0",
            onfocusin: move |_| focused.set(true),
            onfocusout: move |_| focused.set(false),
            label { 
                position: "absolute",
                transform_origin: "top left",
                transform: format!("translate({}px, {}px) scale({})",{transform.get_value().x}, {transform.get_value().y}, {transform.get_value().scale}),
                top: "0px",
                left: "0px",
                {props.label} 
            }
            input {
                style:"caret-color: #D0BCFF; background: transparent; border: none; outline: none;",
                value: props.value,
            }
        }
    )
}