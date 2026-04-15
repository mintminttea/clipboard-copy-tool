fn main() {
    if std::env::var("CARGO_CFG_WINDOWS").is_ok() {
        let mut res = winres::WindowsResource::new();
        res.set_icon("assets/favicon.ico"); // 執返你粒 icon 路徑
        res.compile().unwrap();
    }
}