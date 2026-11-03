# gpui-template

[English](README.md) | 简体中文

集成 [`gpui-router`](https://github.com/justjavac/gpui-router) 页面导航的最小 GPUI + OpenHarmony（OHOS）Rust 应用模板。

## 依赖版本

参考 [NearSend](https://github.com/richerfu/nearsend) 的鸿蒙接入方式，使用已发布的依赖包：

| 依赖 | 版本 |
| --- | --- |
| `gpui`（实际包名为 `gpui-pre`） | `0.3.11` |
| `gpui-ohos` | `0.0.1-beta.0` |
| `gpui-router` | `0.7.0` |
| OpenHarmony Ability（Rust 与 ArkTS） | `1.0.0-beta.5` |
| NAPI OHOS | `1.2.0` |

版本统一维护在 `Cargo.toml` 的 `[workspace.dependencies]` 中。路由仅启用 `gpui-pre` 后端，与 `gpui-ohos` 共用 GPUI 类型。升级时需要同步 Rust 与 ArkTS 的 Ability 版本。

## 构建与运行

1. 编译 Rust 原生模块（`aarch64-unknown-linux-ohos`）并输出至鸿蒙工程目录：

   ```bash
   ohrs build --arch aarch --dist ohos/entry/libs -- --locked
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
      gpui_router::init(cx);
      // 打开窗口并挂载根视图
  });
  ```

- **页面路由（[src/app.rs](src/app.rs)）**：`AppView` 通过 `Outlet` 挂载共用的导航布局。`/` 为计数器首页，`/about` 演示历史返回，未匹配的路径展示兜底页。首页实体由根视图持有，切换页面后计数保持不变。

  在 `AppView::render` 的布局路由下增加页面：

  ```rust
  Route::new().path("settings").element(|_, _| div().child("Settings"))
  ```

  使用 `Link` / `NavLink` 跳转到 `/settings` 等绝对路径，或通过 `use_navigate(window, cx)` 编程导航。路由模式相对于布局填写，例如 `settings`。导航后路由会自动刷新窗口。

- **ArkTS 入口（[EntryAbility.ets](ohos/entry/src/main/ets/entryability/EntryAbility.ets)）**：继承 `@ohos-rs/ability` 的 `NativeAbility`，设置 `moduleName = "gpui_template"` 并在 `bridgePlugins` 中注册桥接插件。

## License

[Apache-2.0](LICENSE) · [第三方依赖声明](THIRD_PARTY_LICENSES.md)
