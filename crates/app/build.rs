fn main() {
    if std::env::var("CARGO_CFG_TARGET_OS").is_ok_and(|os| os == "windows") {
        winresource::WindowsResource::new()
            .set_icon("assets/copyanywhere.ico")
            .compile()
            .expect("failed to embed the program icon");
    }
}
