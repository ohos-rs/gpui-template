# gpui-template

Minimal Rust-only template for running a GPUI app in OpenHarmony (OHOS).

## Rust layout

```text
.
├── Cargo.toml
├── build.rs
└── src
    ├── app.rs
    └── lib.rs
```

## What is included

- `src/lib.rs`: OHOS ability entry and GPUI app bootstrap.
- `src/app.rs`: minimal `Render` view (`HelloView`).
- `build.rs`: OHOS napi build setup and `c++_shared` link line.

## License (important)

- This template: Apache-2.0. See `LICENSE`.
- `gpui` crate license: Apache-2.0.
- `gpui-component` crate license: Apache-2.0.

`gpui-component` is not required by this minimal template, but its license is documented because GPUI projects commonly add it.

See `THIRD_PARTY_LICENSES.md` for source/commit details.
