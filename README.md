# gpui-template

一个最小可用的 GPUI + OpenHarmony（OHOS）Rust 模板，包含：

- `gpui-component` 初始化与 `gpui-component-assets` 资源注入
- `gpui-router` 结构体路由渲染
- 简单“返回上一级”导航逻辑（用于演示 GPUI 适配）

## 依赖库声明

本项目重点使用以下第三方库：

- `gpui`（Apache-2.0）
- `gpui-component`（Apache-2.0）
- `gpui-router`（MIT）
- `rust-embed`（MIT）

完整信息见 `THIRD_PARTY_LICENSES.md`。

## 目录结构

```text
.
├── Cargo.toml
├── build.rs
└── src
    ├── app
    │   ├── mod.rs
    │   ├── navigation.rs
    │   └── pages.rs
    └── lib.rs
```

## 环境要求

- Rust（建议 stable）
- 已安装 OHOS Rust 目标（至少一个）
- 已安装并可用 `ohrs` 工具链

可选检查命令：

```bash
rustup target list | grep ohos
ohrs --version
```

## 编译

项目根目录执行：

```bash
ohrs build --arch aarch
```

如果你要构建其他架构，请替换 `--arch` 参数。

## 使用说明

构建并安装到设备后，启动应用可看到如下演示流程：

1. 首页（`/`）
2. 点击“进入详情页”进入 `/detail`
3. 点击“进入下一级详情”进入 `/detail/sub`
4. 点击“返回上一级”回退到上一页

顶部也提供了全局“返回上一级”按钮，行为一致。

## 关键代码位置

- 入口与初始化：[src/lib.rs](src/lib.rs)
- 路由壳层与 `Routes/Route` 定义：[src/app/mod.rs](src/app/mod.rs)
- 返回逻辑（history 栈）：[src/app/navigation.rs](src/app/navigation.rs)
- 页面结构体组件：[src/app/pages.rs](src/app/pages.rs)

## License

- 本项目：Apache-2.0，见 `LICENSE`
- 第三方依赖说明：见 `THIRD_PARTY_LICENSES.md`
