use gpui::{App, Application, WindowOptions};
use log::LevelFilter;
use ohos_hilog_binding::log::Config;
use openharmony_ability::OpenHarmonyApp;

mod app;

#[openharmony_ability_derive::ability]
pub fn openharmony_app(app: OpenHarmonyApp) {
    ohos_hilog_binding::log::init_once(Config::default().with_max_level(LevelFilter::Info));

    Application::with_platform(gpui_platform::current_platform(false))
        .with_ohos_app(app)
        .run(|cx: &mut App| {
            cx.open_window(WindowOptions::default(), |_window, cx| {
                cx.new(|_| app::HelloView)
            })
            .expect("failed to open GPUI window");

            cx.activate(true);
        });
}
