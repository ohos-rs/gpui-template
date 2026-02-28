mod navigation;
mod pages;

use gpui::{div, prelude::*, App, Context, IntoElement, Render, RenderOnce, Window};
use gpui_component::{button::Button, h_flex, v_flex};
use gpui_router::{use_location, IntoLayout, Outlet, Route, Routes};

pub fn init(cx: &mut App) {
    navigation::init(cx);
}

pub struct AppView;

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        div().size_full().child(
            Routes::new().basename("/").child(
                Route::new()
                    .layout(ShellLayout::new())
                    .child(Route::new().index().element(|_, _| pages::home()))
                    .child(Route::new().path("detail").element(|_, _| pages::detail()))
                    .child(
                        Route::new()
                            .path("detail/sub")
                            .element(|_, _| pages::detail_sub()),
                    )
                    .child(
                        Route::new()
                            .path("{*not_found}")
                            .element(|_, _| pages::not_found()),
                    ),
            ),
        )
    }
}

#[derive(IntoElement, IntoLayout)]
struct ShellLayout {
    outlet: Outlet,
}

impl ShellLayout {
    fn new() -> Self {
        Self {
            outlet: Outlet::new(),
        }
    }
}

impl RenderOnce for ShellLayout {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pathname = use_location(cx).pathname.clone();

        v_flex()
            .size_full()
            .p_4()
            .gap_4()
            .bg(gpui::rgb(0x0b1220))
            .text_color(gpui::white())
            .child(
                h_flex()
                    .justify_between()
                    .items_center()
                    .child(div().text_sm().child(format!("当前路由: {}", pathname)))
                    .child(
                        Button::new("shell-back")
                            .label("返回上一级")
                            .on_click(|_, _, cx| navigation::navigate_back(cx)),
                    ),
            )
            .child(self.outlet)
    }
}
