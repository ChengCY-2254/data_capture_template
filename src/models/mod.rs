use proc_macro_workshop::Builder;

mod impls;

#[derive(Debug, Builder, Clone, Eq, PartialEq)]
pub struct Chapters {
    pub name: String,
    pub content: String,
    pub prev_page: Option<String>,
    pub next_page: Option<String>,
}

#[derive(Debug, Builder, Clone, Eq, PartialEq)]
pub struct Book {
    pub book_name: String,
    pub author: String,
    pub update_time: Option<String>,
    pub latest_chapter_link: Option<String>,
    pub chapters_len: usize,
}

#[derive(Debug, PartialEq, Eq)]
pub struct Directory {
    pub book_name: String,
    ///href data
    /// 这里的数据需要自定义好数据，也就是自行排序
    pub inner_data: Vec<ChapterLink>,
}

#[derive(Debug, PartialEq, Eq, Ord, PartialOrd)]
pub struct ChapterLink {
    pub href: String,
    pub title: String,
    ///用于排序，下载时会使用到
    pub id: usize,
}