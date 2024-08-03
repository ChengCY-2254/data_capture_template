use super::*;
use crate::traits::{Driver, ParseWith};

impl TryFrom<&Driver> for Chapters {
    type Error = ();

    fn try_from(value: &Driver) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&Driver> for Book {
    type Error = ();

    fn try_from(value: &Driver) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl TryFrom<&Driver> for Directory {
    type Error = ();

    fn try_from(value: &Driver) -> Result<Self, Self::Error> {
        todo!()
    }
}

impl ParseWith for Chapters {
    type Error = ();
    type OutPut = Chapters;

    async fn parse_with(_driver: &'_ Driver) -> Result<Self::OutPut, Self::Error> {
        todo!()
    }

}

impl ParseWith for Book {
    type OutPut = Self;
    type Error = ();


    async fn parse_with(_driver: &'_ Driver) -> Result<Self::OutPut, Self::Error> {
        todo!()
    }
}

impl ParseWith for Directory {
    type OutPut = Directory;
    type Error = ();


    async fn parse_with(_driver: &'_ Driver) -> Result<Self::OutPut, Self::Error> {
        todo!()
    }
}
