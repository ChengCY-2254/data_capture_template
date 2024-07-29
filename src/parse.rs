use thirtyfour::{Capabilities, DesiredCapabilities};

///从字符串中选择用来抓取的浏览器
#[inline]
fn select_browser(text: &str) -> Result<Capabilities, &'static str> {
    match text.to_lowercase().as_str() {
        "chrome" => Ok(DesiredCapabilities::chrome().into()),
        "chromium" => Ok(DesiredCapabilities::chromium().into()),
        "edge" => Ok(DesiredCapabilities::edge().into()),
        "firefox" => Ok(DesiredCapabilities::firefox().into()),
        "safari" => Ok(DesiredCapabilities::safari().into()),
        "opera" => Ok(DesiredCapabilities::opera().into()),
        "ie" => Ok(DesiredCapabilities::internet_explorer().into()),
        _ => Err("missing browser"),
    }
}
#[inline]
pub async fn get_driver(
    address: &str,
    browser: &str,
) -> Result<thirtyfour::WebDriver, &'static str> {
    let capabilities = select_browser(browser)?;
    thirtyfour::WebDriver::new(address, capabilities)
        .await
        .map_err(|_| "连接WebDriver错误，请检查参数是否正确或对应的WebDriver是否已开启")
}