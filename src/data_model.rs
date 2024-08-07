#![allow(unused)]
use proc_macro_workshop::Builder;

#[derive(Debug, Builder, Clone)]
pub struct Chapters {
    pub chapters_name: String,
    pub chapters_content: String,
}

#[derive(Debug, Builder, PartialEq, Eq)]
pub struct Book {
    pub book_name: String,
    pub author: String,
    pub update_time: String,
    pub latest_chapter_link: String,
    pub chapters_len: usize,
}


#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    pub book_name: String,
    pub inner_data: Vec<ChapterLink>,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChapterLink {
    pub href: String,
    pub title: String,
    pub id: usize,
}
