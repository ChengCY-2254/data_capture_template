use std::path::Path;
use crate::parse::DownloadMode;

pub struct ExampleHandler;

impl ExampleHandler {
    pub(crate) async fn run(&self, address: &str, download_path: &Path, proxy_str: Option<&str>, mode: DownloadMode, speed: Option<f32>) -> Result<(), Box<dyn std::error::Error>> {
        todo!()
    }
}