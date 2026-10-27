# gpui-template

一个最小可用的 GPUI + OpenHarmony（OHOS）Rust 应用模板，通过 [`gpui-ohos`](https://github.com/ohos-rs/gpui-ohos) 与 [`openharmony-ability`](https://github.com/harmony-contrib/openharmony-ability) 接入上游 [`gpui`](https://github.com/zed-industries/zed)。

## 特性

- 基于上游 `zed-industries/zed` 最新 `gpui`，无需额外 `[patch]` 或第三方路由补丁。
- 通过 `gpui-ohos` 注入 OpenHarmony 平台后端（`Application::with_platform` + `run_embedded`）。
- 配套完整的 `ohos` Stage 模型工程与 `@ohos-rs/ability` Bridge 插件配置，开箱即用。

## 目录结构

```text
.
├── Cargo.toml
├── build.rs
├── src
│   └── lib.rs               # Rust 入口与 GPUI 视图定义
└── ohos                     # OpenHarmony 应用工程
    ├── build-profile.json5
    └── entry
        ├── libs/arm64-v8a   # 原生动态库输出目录
        ├── oh-package.json5 # @ohos-rs/ability 插件依赖
        └── src/main/ets/entryability/EntryAbility.ets
```

## 环境要求

- Rust 工具链及 OHOS 目标（如 `aarch64-unknown-linux-ohos`）
- [`ohrs`](https://github.com/ohos-rs/ohrs) 构建工具
- DevEco Studio（含 OpenHarmony SDK、`ohpm`、`hvigorw`、`hdc`）

检查命令：

```bash
rustup target add aarch64-unknown-linux-ohos
ohrs --version
```

## 构建与运行

### 1. 编译 Rust 原生模块

在项目根目录执行以下命令，编译 `libgpui_template.so` 与 `libc++_shared.so` 并拷贝到鸿蒙工程的 `libs` 目录：

```bash
ohrs build --arch aarch --dist ohos/entry/libs
```

### 2. 安装依赖与打包运行

使用 DevEco Studio 打开 `ohos` 目录，在 `File -> Project Structure -> Signing Configs` 中配置真机签名后直接点击运行；或通过命令行完成安装与打包：

```bash
cd ohos
ohpm install --all
hvigorw assembleHap --mode module -p product=default
hdc install entry/build/default/outputs/default/entry-default-signed.hap
hdc shell aa start -a EntryAbility -b com.richer.myapplication
```

## 核心接入说明

1. **Rust 入口（[src/lib.rs](src/lib.rs)）**
   使用 `#[openharmony_ability_derive::ability]` 导出 Ability 入口，通过 `gpui_ohos::current_platform(app, false)` 创建平台实例，并调用 `application.run_embedded(...)` 启动窗口、将返回的 `ApplicationHandle` 保存在线程局部存储中保活：

   ```rust
   let application = Application::with_platform(gpui_ohos::current_platform(app, false));
   let application_handle = application.run_embedded(move |cx: &mut App| {
       // 打开窗口并挂载根视图
   });
   ```

2. **ArkTS Ability 入口（[EntryAbility.ets](ohos/entry/src/main/ets/entryability/EntryAbility.ets)）**
   继承 `@ohos-rs/ability` 的 `NativeAbility`，指定 `moduleName = "gpui_template"`（对应 `libgpui_template.so`），并在 `bridgePlugins` 中注册 `gpui-ohos` 所需的窗口、剪贴板、文件、菜单、通知与系统状态插件。

## License

- 本项目：Apache-2.0，见 [LICENSE](LICENSE)
- 第三方依赖说明：见 [THIRD_PARTY_LICENSES.md](THIRD_PARTY_LICENSES.md)
