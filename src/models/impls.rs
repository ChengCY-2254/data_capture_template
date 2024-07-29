use super::*;
use crate::traits::ParseDocument;
use async_trait::async_trait;
use thirtyfour::error::WebDriverError;
use thirtyfour::WebDriver;

impl TryFrom<&WebDriver> for Chapters {
    type Error = ();

    fn try_from(value: &WebDriver) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&WebDriver> for Book {
    type Error = ();

    fn try_from(value: &WebDriver) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&WebDriver> for Directory {
    type Error = ();

    fn try_from(value: &WebDriver) -> Result<Self, Self::Error> {
        todo!()
    }
}

#[async_trait]
impl ParseDocument for Chapters {
    type OutPut = Chapters;

    async fn parse(driver: &'_ WebDriver) -> Result<Self::OutPut, WebDriverError> {
        unimplemented!()
    }
}

#[async_trait]
impl ParseDocument for Book {
    type OutPut = Self;

    async fn parse(driver: &'_ WebDriver) -> Result<Self::OutPut, WebDriverError> {
        unimplemented!()
    }
}

#[async_trait]
impl ParseDocument for Directory {
    type OutPut = Directory;

    async fn parse(driver: &'_ WebDriver) -> Result<Self::OutPut, WebDriverError> {
        unimplemented!()
    }
}
