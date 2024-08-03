#![allow(unused)]


pub type Driver = fantoccini::Client;
pub type By<'a> = fantoccini::Locator<'a>;

pub trait ParseWith<T = Driver> {
    type OutPut;
    type Error;
    async fn parse_with(_driver: &'_ T) -> Result<Self::OutPut, Self::Error>;
}
