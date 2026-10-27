# gpui-template

[English](README.md) | 简体中文

基于 [`gpui`](https://github.com/zed-industries/zed) 与 [`gpui-ohos`](https://github.com/ohos-rs/gpui-ohos) 的最小 GPUI + OpenHarmony（OHOS）Rust 应用模板。

## 构建与运行

1. 编译 Rust 原生模块（`aarch64-unknown-linux-ohos`）并输出至鸿蒙工程目录：

   ```bash
   ohrs build --arch aarch --dist ohos/entry/libs
   ```

2. 安装 ArkTS 依赖，随后使用 DevEco Studio 打开 `ohos/` 目录配置真机签名并运行：

   ```bash
   cd ohos
   ohpm install --all
   ```

## 核心接入

- **Rust 入口（[src/lib.rs](src/lib.rs)）**：通过 `gpui_ohos::current_platform` 注入 OpenHarmony 平台后端，并持有 `run_embedded` 返回的 `ApplicationHandle`：

  ```rust
  let application = Application::with_platform(gpui_ohos::current_platform(app, false));
  let application_handle = application.run_embedded(move |cx: &mut App| {
      // 打开窗口并挂载根视图
  });
  ```

- **ArkTS 入口（[EntryAbility.ets](ohos/entry/src/main/ets/entryability/EntryAbility.ets)）**：继承 `@ohos-rs/ability` 的 `NativeAbility`，设置 `moduleName = "gpui_template"` 并在 `bridgePlugins` 中注册桥接插件。

## License

[Apache-2.0](LICENSE) · [第三方依赖声明](THIRD_PARTY_LICENSES.md)
