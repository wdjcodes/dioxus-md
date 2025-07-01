use dioxus::{dioxus_core, prelude::*};

use crate::text_fields::text_field_attrs::TextFieldAttrs;

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
            position: "relative",
            display: "flex",
            font_family: "sans-serif",
            border_bottom: "2px solid #999",
            label { {props.label} }
            input {
                value: props.value,
            }
        }
    )
}