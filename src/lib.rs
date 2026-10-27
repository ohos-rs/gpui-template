use std::cell::RefCell;

use gpui::{
    App, AppContext, Application, ApplicationHandle, Bounds, Context, IntoElement, Render, Window,
    WindowBounds, WindowOptions, div, prelude::*, px, rgb, size,
};
use log::LevelFilter;
use ohos_hilog_binding::log::Config;
use openharmony_ability::OpenHarmonyApp;

thread_local! {
    #[allow(clippy::missing_const_for_thread_local)]
    static APPLICATION: RefCell<Option<ApplicationHandle>> = const { RefCell::new(None) };
}

struct AppView {
    count: usize,
}

impl Render for AppView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .size_full()
            .items_center()
            .justify_center()
            .gap_4()
            .bg(rgb(0x0b1220))
            .text_color(rgb(0xf4f4f5))
            .child(div().text_xl().child("Hello, GPUI on OpenHarmony!"))
            .child(
                div()
                    .id("counter-button")
                    .px_4()
                    .py_2()
                    .rounded_md()
                    .bg(rgb(0x2563eb))
                    .child(format!("Count: {}", self.count))
                    .on_click(cx.listener(|this, _event, _window, cx| {
                        this.count += 1;
                        cx.notify();
                    })),
            )
    }
}

#[openharmony_ability_derive::ability]
pub fn openharmony_app(app: OpenHarmonyApp) {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Info));

    let inner_app = app.clone();
    let application = Application::with_platform(gpui_ohos::current_platform(app, false));

    let application_handle = application.run_embedded(move |cx: &mut App| {
        let info = inner_app.content_rect();
        let default_size = size(px(info.width as f32), px(info.height as f32));
        let bounds = Bounds::centered(None, default_size, cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(|_| AppView { count: 0 }),
        )
        .expect("failed to open GPUI window");

        cx.activate(true);
    });

    APPLICATION.with(|application| {
        application.replace(Some(application_handle));
    });
}
