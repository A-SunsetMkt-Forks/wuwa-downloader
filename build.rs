fn main() {
    #[cfg(windows)]
    {
        let mut res = winres::WindowsResource::new();
        res.set_icon("cartethyia.ico");
        res.compile().unwrap();
    }
}
