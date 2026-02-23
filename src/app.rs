use gpui::{div, prelude::*, Context, IntoElement, Window};

pub struct HelloView;

impl Render for HelloView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child("Hello GPUI on OHOS (Rust)")
    }
}
