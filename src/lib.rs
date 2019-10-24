use std::fmt;
use reqwest;

pub mod routes {
    pub static BASE_URL: &str = "https://api.unsplash.com/";
    pub static SEARCH_PHOTOS: &str = "search/photos";
}

pub struct Unsplash<'a> {
    creds: Creds<'a>
}

#[derive(Copy, Clone)]
struct Creds<'a> {
    access_key: &'a str,
    secret_key: &'a str,
}

impl<'a> Creds<'a> {
    fn get_access_key_param(&self) -> String {
        format!("access_key={}", self.access_key)
    }
}

impl<'a> Unsplash<'a> {
    pub fn new(access_key: &'a str, secret_key: &'a str) -> Unsplash<'a> {
        Unsplash {
            creds: Creds {
                access_key,
                secret_key,
            }
        }
    }

    pub fn search_photos<'b>(&self, query: &'b str) -> SearchPhotosBuilder<'b> {
        SearchPhotosBuilder::new(query)
    }
}

pub struct SearchPhotosBuilder<'a> {
    query: &'a str,
    page: Option<u32>,
    per_page: Option<u32>,
    collections: Option<Vec<u32>>,
    orientation: Option<Orientation>
}

impl<'a> SearchPhotosBuilder<'a> {
    fn new(query: &'a str) -> SearchPhotosBuilder<'a> {
        SearchPhotosBuilder {
            query,
            page: None,
            per_page: None,
            collections: None,
            orientation: None,
        }
    }

    pub fn send(self) -> reqwest::Result<reqwest::Response> {
        // let ak_param = self.creds.get_access_key_param();
        // let url = format!("{}{}?query={}&{}", routes::BASE_URL, routes::SEARCH_PHOTOS, self.query, ak_param);
        reqwest::get("https://www.rust-lang.org")
    }

    pub fn page(&mut self, page: u32) -> &mut Self {
        self.page = Some(page);
        self
    }
}

enum Orientation {
    Landscape,
    Portrait,
    Squarish,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Landscape => write!(f, "{}", "landscape"),
            Orientation::Portrait =>  write!(f, "{}", "portrait"),
            Orientation::Squarish =>  write!(f, "{}", "squarish"),
        }
    }
}