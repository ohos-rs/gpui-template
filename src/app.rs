use gpui::{AppContext, Context, Entity, IntoElement, Render, Window, div, prelude::*, rgb};
use gpui_router::{Link, NavLink, Outlet, Route, Routes, use_navigate};

pub(crate) struct AppView {
    // Retain the page entity so navigating away does not reset the counter.
    home: Entity<HomePage>,
}

impl AppView {
    pub(crate) fn new(cx: &mut Context<Self>) -> Self {
        Self {
            home: cx.new(|_| HomePage { count: 0 }),
        }
    }

    fn layout() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .p_4()
            .bg(rgb(0x0b1220))
            .text_color(rgb(0xf4f4f5))
            .child(
                div()
                    .flex()
                    .gap_4()
                    .child(Self::nav_link("/", "Home"))
                    .child(Self::nav_link("/about", "About")),
            )
            .child(Outlet::new())
    }

    fn nav_link(path: &'static str, label: &'static str) -> NavLink {
        NavLink::new()
            .to(path)
            .end(true)
            .px_4()
            .py_2()
            .rounded_md()
            .cursor_pointer()
            .active(|style| style.bg(rgb(0x2563eb)))
            .child(label)
    }

    fn about_page() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(div().text_xl().child("About this template"))
            .child("GPUI + OpenHarmony + gpui-router")
            .child(
                div()
                    .id("back-button")
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x2563eb))
                    .cursor_pointer()
                    .child("Back")
                    .on_click(|_, window, cx| use_navigate(window, cx).back()),
            )
    }

    fn not_found_page() -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(div().text_xl().child("Page not found"))
            .child(
                Link::new()
                    .to("/")
                    .replace(true)
                    .cursor_pointer()
                    .child("Go to Home"),
            )
    }
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
        let home = self.home.clone();

        // Keep links and the outlet inside the route's per-window context.
        Routes::new().child(
            Route::new()
                .element(|_, _| Self::layout())
                .child(Route::new().index().element(move |_, _| home.clone()))
                .child(
                    Route::new()
                        .path("about")
                        .element(|_, _| Self::about_page()),
                )
                .child(
                    Route::new()
                        .path("*")
                        .element(|_, _| Self::not_found_page()),
                ),
        )
    }
}

struct HomePage {
    count: usize,
}

impl Render for HomePage {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .items_center()
            .gap_4()
            .child(div().text_xl().child("Hello, GPUI on OpenHarmony!"))
            .child(
                div()
                    .id("counter-button")
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x2563eb))
                    .cursor_pointer()
                    .child(format!("Count: {}", self.count))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.count += 1;
                        cx.notify();
                    })),
            )
    }
}
