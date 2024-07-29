use async_trait::async_trait;

///```rust
/// struct Demo;
///
/// #[async_trait]
/// impl ParseDocument for Demo {
///     type OutPut = ();
///
///     async fn parse(_driver: &'_ WebDriver) -> Result<Self::OutPut, WebDriverError> {
///         todo!()
///     }
/// }
///
/// ```
/// 用于解析网页文档并转换为自己的数据模型
/// 
///
#[async_trait]
pub trait ParseDocument {
    type OutPut;
    async fn parse(
        driver: &'_ thirtyfour::WebDriver,
    ) -> Result<Self::OutPut, thirtyfour::error::WebDriverError>;
}
