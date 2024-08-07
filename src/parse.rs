#![allow(unused)]
use fantoccini::wd::Capabilities;
use serde_json::json;

fn parse_proxy_caps(
    caps: &mut Capabilities,
    proxy_str: Option<&str>,
) -> Result<(), &'static str> {
    if let Some(proxy_str) = proxy_str {
        //socks5代理
        let proxy_obj = if proxy_str.starts_with("socks5://") {
            let proxy_str = proxy_str.replace("socks5://","");
            json!({
                "proxyType": "manual",
                "socksProxy": proxy_str,
                "socksVersion":5
            })
        } else {
            eprintln!("不是一个有效的socks5代理字符串，请检查你的配置");
            std::process::exit(1);
        };
        caps.insert("proxy".to_string(), proxy_obj);
    }
    Ok(())
}
#[inline]
pub async fn get_driver(
    address: &str,
    proxy_str: Option<&str>,
) -> Result<fantoccini::Client, Box<dyn std::error::Error>> {
    let mut caps = Capabilities::new();
    parse_proxy_caps(&mut caps, proxy_str)?;
    fantoccini::ClientBuilder::native()
        .capabilities(caps)
        .connect(address)
        .await
        .map_err(|e| format!("连接到WebDriver出现错误，请检查参数是否正确 {}", e).into())
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum DownloadMode {
    Chapter(String),
    Directory(String),
}