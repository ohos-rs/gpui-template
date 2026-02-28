use super::navigation;
use gpui::{div, prelude::*, App, IntoElement, RenderOnce, Window};
use gpui_component::{button::Button, v_flex};
use gpui_router::use_location;

pub fn home() -> impl IntoElement {
    HomePage
}

pub fn detail() -> impl IntoElement {
    DetailPage
}

pub fn detail_sub() -> impl IntoElement {
    DetailSubPage
}

pub fn not_found() -> impl IntoElement {
    NotFoundPage
}

#[derive(IntoElement)]
struct HomePage;

impl RenderOnce for HomePage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(div().text_xl().child("首页"))
            .child(
                div()
                    .text_sm()
                    .child("这是最小结构：gpui-component + gpui-router + 返回逻辑。"),
            )
            .child(
                Button::new("go-detail")
                    .label("进入详情页")
                    .on_click(|_, _, cx| navigation::navigate_to("/detail", cx)),
            )
    }
}

#[derive(IntoElement)]
struct DetailPage;

impl RenderOnce for DetailPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pathname = use_location(cx).pathname.clone();

        v_flex()
            .gap_3()
            .child(div().text_xl().child("详情页"))
            .child(div().text_sm().child(format!("当前路径: {}", pathname)))
            .child(
                Button::new("go-detail-sub")
                    .label("进入下一级详情")
                    .on_click(|_, _, cx| navigation::navigate_to("/detail/sub", cx)),
            )
            .child(
                Button::new("go-back-detail")
                    .label("返回上一级")
                    .on_click(|_, _, cx| navigation::navigate_back(cx)),
            )
    }
}

#[derive(IntoElement)]
struct DetailSubPage;

impl RenderOnce for DetailSubPage {
    fn render(self, _window: &mut Window, cx: &mut App) -> impl IntoElement {
        let pathname = use_location(cx).pathname.clone();

        v_flex()
            .gap_3()
            .child(div().text_xl().child("详情子页"))
            .child(div().text_sm().child(format!("当前路径: {}", pathname)))
            .child(
                Button::new("go-back-sub")
                    .label("返回上一级")
                    .on_click(|_, _, cx| navigation::navigate_back(cx)),
            )
    }
}

#[derive(IntoElement)]
struct NotFoundPage;

impl RenderOnce for NotFoundPage {
    fn render(self, _window: &mut Window, _cx: &mut App) -> impl IntoElement {
        v_flex()
            .gap_3()
            .child(div().text_xl().child("404"))
            .child(div().text_sm().child("路由不存在"))
            .child(
                Button::new("go-home")
                    .label("回到首页")
                    .on_click(|_, _, cx| navigation::navigate_to("/", cx)),
            )
    }
}
