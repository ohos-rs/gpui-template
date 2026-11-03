use std::cell::RefCell;

use gpui::{
    App, AppContext, Application, ApplicationHandle, Bounds, WindowBounds, WindowOptions, px, size,
};
use log::LevelFilter;
use ohos_hilog_binding::log::Config;
use openharmony_ability::OpenHarmonyApp;

mod app;

use app::AppView;

thread_local! {
    #[allow(clippy::missing_const_for_thread_local)]
    static APPLICATION: RefCell<Option<ApplicationHandle>> = const { RefCell::new(None) };
}

#[openharmony_ability_derive::ability]
pub fn openharmony_app(app: OpenHarmonyApp) {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Info));

    let inner_app = app.clone();
    let application = Application::with_platform(gpui_ohos::current_platform(app, false));

    let application_handle = application.run_embedded(move |cx: &mut App| {
        gpui_router::init(cx);

        let info = inner_app.content_rect();
        let default_size = size(px(info.width as f32), px(info.height as f32));
        let bounds = Bounds::centered(None, default_size, cx);

        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                ..Default::default()
            },
            |_, cx| cx.new(AppView::new),
        )
        .expect("failed to open GPUI window");

        cx.activate(true);
    });

    APPLICATION.with(|application| {
        application.replace(Some(application_handle));
    });
}
