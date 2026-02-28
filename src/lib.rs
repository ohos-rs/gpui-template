use gpui::{px, size, App, AppContext, Application, Bounds, WindowBounds, WindowOptions};
use log::LevelFilter;
use ohos_hilog_binding::log::Config;
use openharmony_ability::OpenHarmonyApp;

mod app;

#[openharmony_ability_derive::ability]
pub fn openharmony_app(app: OpenHarmonyApp) {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Info));

    let inner_app = app.clone();
    Application::with_platform(gpui_platform::current_platform(false))
        .with_assets(gpui_component_assets::Assets)
        .with_ohos_app(app)
        .run(move |cx: &mut App| {
            gpui_component::init(cx);
            gpui_router::init(cx);
            app::init(cx);

            let info = inner_app.content_rect();
            let default_size = size(px(info.width as f32), px(info.height as f32));
            let bounds = Bounds::centered(None, default_size, cx);

            cx.open_window(
                WindowOptions {
                    window_bounds: Some(WindowBounds::Windowed(bounds)),
                    ..Default::default()
                },
                |window, cx| {
                    let view = cx.new(|_| app::AppView);
                    cx.new(|cx| {
                        gpui_component::Root::new(view, window, cx).window_shadow_size(px(0.))
                    })
                },
            )
            .expect("failed to open GPUI window");

            cx.activate(true);
        });
}
