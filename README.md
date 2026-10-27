# gpui-template

English | [简体中文](README.zh-CN.md)

Minimal GPUI + OpenHarmony (OHOS) Rust template powered by [`gpui`](https://github.com/zed-industries/zed) and [`gpui-ohos`](https://github.com/ohos-rs/gpui-ohos).

## Build & Run

1. Build the Rust native module (`aarch64-unknown-linux-ohos`) into the OHOS project:

   ```bash
   ohrs build --arch aarch --dist ohos/entry/libs
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
      // Open window and mount root view
  });
  ```

- **ArkTS entry ([EntryAbility.ets](ohos/entry/src/main/ets/entryability/EntryAbility.ets))**: Extend `NativeAbility` from `@ohos-rs/ability`, set `moduleName = "gpui_template"`, and register the bridge plugins in `bridgePlugins`.

## License

[Apache-2.0](LICENSE) · [Third-Party Notices](THIRD_PARTY_LICENSES.md)
