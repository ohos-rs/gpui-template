use gpui::{div, prelude::*, rgb, Context, IntoElement, Window};

pub struct HelloView;

impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .size_full()
            .bg(rgb(0x0b1220))
            .text_color(rgb(0xffffff))
            .flex()
            .justify_center()
            .items_center()
            .child(
                div()
                    .text_xl()
                    .font_weight(gpui::FontWeight::BOLD)
                    .child("Hello GPUI on OHOS (Rust)"),
            )
    }
}
