fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows") {
        // Load static imports from System32 only, so a proxy DLL next to the exe (e.g. Clean Ops) is ignored.
        if std::env::var("CARGO_CFG_TARGET_ENV").is_ok_and(|env| env == "msvc") {
            println!("cargo:rustc-link-arg-bins=/DEPENDENTLOADFLAG:0x800");
        }
        winresource::WindowsResource::new()
            .set_icon("assets/copyanywhere.ico")
            .compile()
            .expect("failed to embed the program icon");
    }
}
