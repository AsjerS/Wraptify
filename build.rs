fn main() -> std::io::Result<()> {
    if std::env::var("CARGO_CFG_TARGET_OS").unwrap() == "windows" {
        let mut res = winres::WindowsResource::new();
        res.set_icon("wraptify.ico");
        res.compile()?;
    }
    Ok(())
}
