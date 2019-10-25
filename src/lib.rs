use std::fmt;
use reqwest;

pub mod routes {
    pub static BASE_URL: &str = "https://api.unsplash.com/";
    pub static SEARCH_PHOTOS: &str = "search/photos";
}

pub struct Unsplash {
    creds: Creds
}

#[derive(Clone, Debug)]
struct Creds {
    access_key: String,
    secret_key: String,
}

impl Creds {
    fn get_access_key_param(&self) -> String {
        format!("client_id={}", self.access_key)
    }
}

impl Unsplash {
    pub fn new(access_key: &str, secret_key: &str) -> Unsplash {
        Unsplash {
            creds: Creds {
                access_key: access_key.to_owned(),
                secret_key: secret_key.to_owned(),
            }
        }
    }

    pub fn search_photos(&self, query: &str) -> SearchPhotosBuilder {
        SearchPhotosBuilder::new(query, self.creds.clone())
    }
}

#[derive(Debug)]
pub struct SearchPhotosBuilder {
    creds: Creds,
    query: String,
    page: Option<u32>,
    per_page: Option<u32>,
    collections: Option<Vec<u32>>,
    orientation: Option<Orientation>
}

impl SearchPhotosBuilder {
    fn new(query: &str, creds: Creds) -> SearchPhotosBuilder {
        SearchPhotosBuilder {
            creds,
            query: query.to_owned(),
            page: None,
            per_page: None,
            collections: None,
            orientation: None,
        }
    }

    pub fn send(self) -> reqwest::Result<reqwest::Response> {
        let ak_param = self.creds.get_access_key_param();
        let url = format!("{}{}?query={}&{}", routes::BASE_URL, routes::SEARCH_PHOTOS, self.query, ak_param);
        println!("req: {}", url);
        reqwest::get(&url)
    }

    pub fn page(&mut self, page: u32) -> &mut Self {
        self.page = Some(page);
        self
    }
}

#[derive(Debug)]
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