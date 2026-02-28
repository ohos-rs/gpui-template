use gpui::{App, Global, SharedString};
use gpui_router::{use_navigate, RouterState};

#[derive(Default)]
pub struct NavigationHistory {
    stack: Vec<SharedString>,
}

impl Global for NavigationHistory {}

pub fn init(cx: &mut App) {
    cx.set_global(NavigationHistory::default());
}

pub fn navigate_to(path: impl Into<SharedString>, cx: &mut App) {
    let target = path.into();
    let current = cx.global::<RouterState>().location.pathname.clone();

    if current == target {
        return;
    }

    cx.global_mut::<NavigationHistory>().stack.push(current);

    let mut navigate = use_navigate(cx);
    navigate(target);
}

pub fn navigate_back(cx: &mut App) {
    let target = cx
        .global_mut::<NavigationHistory>()
        .stack
        .pop()
        .unwrap_or_else(|| "/".into());

    let current = cx.global::<RouterState>().location.pathname.clone();
    if current == target {
        return;
    }

    let mut navigate = use_navigate(cx);
    navigate(target);
}
