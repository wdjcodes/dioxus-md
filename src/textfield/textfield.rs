use std::rc::Rc;

use dioxus::prelude::*;
use dioxus_motion::prelude::*;
use crate::{make_spring, TextFieldAttrs};

const STYLE_SHEET: Asset = asset!("/src/textfield/styles.css");
#[component]
pub fn TextField(
    mut props: TextFieldAttrs
) -> Element {

    let focus_in = Transform{x: 16.0, y: 8.0, scale: 0.75, rotation: 0.0};
    let focus_out = Transform {x: 16.0, y: 16.0, scale: 1.0, rotation: 0.0};
    let mut focused = use_signal(||false);
    let initial_state = if focused() || !props.value.read().is_empty() {
        focus_in
    } else {
        focus_out
    };
    let mut transform = use_motion(initial_state);
    let mut input = use_signal(|| None::<Rc<MountedData>>);
    use_effect(move || {
        let ms = AnimationConfig::new(AnimationMode::Spring(make_spring(3800.0, 25.0, 1.0)));
        if focused() {
            transform.animate_to(focus_in, ms);
            if let Some(i) = input() {
                spawn(async move { i.set_focus(true).await;});
            }
        } else if props.value.read().is_empty() {
            transform.animate_to(focus_out, ms);
        }
    });

    rsx!(
        document::Stylesheet{ href: STYLE_SHEET }
        div {
            class: if props.disabled {
                "textfield disabled"
            } else {
                "textfield"
            },
            tabindex: "0",
            "inert": if props.disabled {
                Some("")
            } else {
                None
            },
            
            onfocusin: move |_| focused.set(true),
            onfocusout: move |_| focused.set(false),
            div{
                class: "background",
            }
            div {
                class: "statelayer",
            }
            label { 
                transform: format!("translate({}px, {}px) scale({})",{transform.get_value().x}, {transform.get_value().y}, {transform.get_value().scale}),
                {props.label} 
            }
            input {
                value: props.value,
                disabled: props.disabled,
                onmounted: move |elem| input.set(Some(elem.data)),
                oninput: move |event| props.value.set(event.value()),
            }
            div{ class: "status"}
        }
    )
}