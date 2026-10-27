# gpui-template

一个最小可用的 GPUI + OpenHarmony（OHOS）Rust 模板，通过 [`gpui-ohos`](https://github.com/ohos-rs/gpui-ohos) 直接接入上游 `gpui`。

## 依赖库声明

本项目使用以下核心库：

- `gpui`（Apache-2.0）
- `gpui-ohos`（MIT OR Apache-2.0）

完整信息见 `THIRD_PARTY_LICENSES.md`。

## 目录结构

```text
.
├── Cargo.toml
├── build.rs
├── ohos
│   └── entry
└── src
    └── lib.rs
```

## 环境要求

- 已安装 OHOS Rust 目标（如 `aarch64-unknown-linux-ohos`）
- 已安装并可用 `ohrs` 工具链

检查命令：

```bash
rustup target list | grep ohos
ohrs --version
```

## 编译

项目根目录执行：

```bash
ohrs build --arch aarch
```

如果要构建其他架构，请替换 `--arch` 参数。

## 关键代码位置

- 入口与窗口初始化：[src/lib.rs](src/lib.rs)
- ArkTS EntryAbility：[ohos/entry/src/main/ets/entryability/EntryAbility.ets](ohos/entry/src/main/ets/entryability/EntryAbility.ets)

## License

- 本项目：Apache-2.0，见 `LICENSE`
- 第三方依赖说明：见 `THIRD_PARTY_LICENSES.md`
