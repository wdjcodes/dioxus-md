use dioxus::{dioxus_core, prelude::*};

use crate::TextFieldAttrs;

#[component]
pub fn TextField(
    props: TextFieldAttrs
) -> Element {
    // let mut is_populated = use_signal(|| !attrs.value.is_empty());
    // let theme = use_theme();

    // let font_size = 16;
    // let spring = use_spring(
    //     if is_populated() {
    //         [10f32, 12f32, 16f32]
    //     } else {
    //         [20., font_size, 24.]
    //     },
    //     Duration::from_millis(50),
    // );

    // let mounted = use_mounted();
    // use_animated(mounted, spring, |[top, font_size, line_height]| {
    //     format!(
    //         r"
    //         position: absolute;
    //         top: {top}px;
    //         left: 20px;
    //         font-size: {font_size}px;
    //         line-height: {line_height}px;
    //     "
    //     )
    // });

    // let background = background.as_deref().unwrap_or(&theme.background_color);
    // let width = width.as_deref().unwrap_or("200px");

    rsx!(
        div {
            background: "#36343B",
            padding_left: "16px",
            padding_right: "16px",
            padding_top: "16px",
            padding_bottom: "16px",
            color: "#CAC4D0",
            border_bottom: "1px solid #CAC4D0",
            // position: "relative",
            display: "flex",
            font_family: "sans-serif",
            label { 
                {props.label} 
            }
            input {
                // background: "transparent",
                // border: "none",
                style:"caret-color: #D0BCFF; background: transparent; border: none; outline: none;",
                value: props.value,
            }
        }
    )
}