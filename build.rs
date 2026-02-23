fn main() {
    napi_build_ohos::setup();
    println!("cargo:rustc-link-lib=dylib=c++_shared");
}
