pub enum Handlers{
    // Add your handlers here
    #[cfg(feature = "example")]
    ExampleHandle(crate::handlers::ExampleHandler)
}

impl std::convert::TryFrom<&str> for Handlers {
    type Error = &'static str;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        #[cfg(feature = "example")]
        if value.starts_with("https://example.com") {
            return Ok(Handlers::ExampleHandle(crate::handlers::ExampleHandler));
        }
        Err("No handler found for this URL")
    }
}

impl Handlers {
    #[allow(unreachable_patterns)]
    pub async fn run(
        self,
        address: &str,
        download_path: &std::path::Path,
        proxy_str: Option<&str>,
        mode: crate::parse::DownloadMode,
        speed: Option<f32>,
    ) -> Result<(), Box<dyn std::error::Error>> {

        match self {
            #[cfg(feature = "example")]
            Handlers::ExampleHandle(handle) => handle.run(address, download_path, proxy_str, mode,speed).await,
            _ => Err("未找到与域名对应的下载器".into()),
        }
    }
}
