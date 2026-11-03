# gpui-template

English | [简体中文](README.zh-CN.md)

Minimal GPUI + OpenHarmony (OHOS) Rust template with [`gpui-router`](https://github.com/justjavac/gpui-router) navigation.

## Dependencies

Uses published packages, following [NearSend](https://github.com/richerfu/nearsend)'s OHOS integration:

| Dependency | Version |
| --- | --- |
| `gpui` (the `gpui-pre` package) | `0.3.11` |
| `gpui-ohos` | `0.0.1-beta.0` |
| `gpui-router` | `0.7.0` |
| OpenHarmony Ability (Rust and ArkTS) | `1.0.0-beta.5` |
| NAPI OHOS | `1.2.0` |

Versions are centralized in `Cargo.toml` under `[workspace.dependencies]`. The router enables only the `gpui-pre` backend, so it shares GPUI types with `gpui-ohos`. Keep the Rust and ArkTS Ability versions aligned when upgrading.

## Build & Run

1. Build the Rust native module (`aarch64-unknown-linux-ohos`) into the OHOS project:

   ```bash
   ohrs build --arch aarch --dist ohos/entry/libs -- --locked
   ```

2. Install ArkTS dependencies, then open `ohos/` in DevEco Studio to configure signing and run on a device:

   ```bash
   cd ohos
   ohpm install --all
   ```

## Integration

- **Rust entry ([src/lib.rs](src/lib.rs))**: Inject the OpenHarmony platform with `gpui_ohos::current_platform` and retain the returned `ApplicationHandle`:

  ```rust
  let application = Application::with_platform(gpui_ohos::current_platform(app, false));
  let application_handle = application.run_embedded(move |cx: &mut App| {
      gpui_router::init(cx);
      // Open window and mount root view
  });
  ```

- **Routes ([src/app.rs](src/app.rs))**: `AppView` mounts a shared navigation layout with an `Outlet`. `/` shows the counter, `/about` demonstrates history navigation, and unmatched paths show a fallback page. The home page entity is retained, preserving its counter across navigation.

  Add pages as children of the layout route in `AppView::render`:

  ```rust
  Route::new().path("settings").element(|_, _| div().child("Settings"))
  ```

  Use `Link` / `NavLink` with absolute destinations such as `/settings`, or `use_navigate(window, cx)` for programmatic navigation. Route patterns are relative to the layout, such as `settings`. The router refreshes the window after navigation.

- **ArkTS entry ([EntryAbility.ets](ohos/entry/src/main/ets/entryability/EntryAbility.ets))**: Extend `NativeAbility` from `@ohos-rs/ability`, set `moduleName = "gpui_template"`, and register the bridge plugins in `bridgePlugins`.

## License

[Apache-2.0](LICENSE) · [Third-Party Notices](THIRD_PARTY_LICENSES.md)
