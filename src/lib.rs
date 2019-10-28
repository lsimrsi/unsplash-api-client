use reqwest;
use std::fmt;
use serde::Deserialize;
use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};

pub mod routes {
    pub const BASE_URL: &str = "https://api.unsplash.com/";
    pub const SEARCH_PHOTOS: &str = "search/photos";
}

pub struct Unsplash {
    client: reqwest::Client,
    access_key: String,
    _secret_key: String,
}

impl Unsplash {
    pub fn new(access_key: &str, secret_key: &str) -> Unsplash {
        Unsplash {
            client: reqwest::Client::new(),
            access_key: access_key.to_owned(),
            _secret_key: secret_key.to_owned(),
        }
    }

    pub fn get<R, O>(&self, required: R, optional: O) -> reqwest::Result<String>
        where
            R: Required,
            O: Optional
    {
        let url = format!("{base}{path}?{required}{optional}{key}",
            base = routes::BASE_URL,
            path = routes::SEARCH_PHOTOS,
            required = required.to_query(),
            optional = optional.to_query(required.get_route()),
            key = self.get_access_key_param());

        println!("url: {}", &url);
        self.client.get(&url).send()?.text()
    }

    fn get_access_key_param(&self) -> String {
        format!("&client_id={}", self.access_key)
    }
}

pub trait Required {
    fn get_route(&self) -> &'static str;
    fn to_query(&self) -> String;
}

pub trait Optional {
    fn to_query(&self, path: &str) -> String;
}

#[derive(Deserialize, Debug)]
pub struct SearchPhotos {
    query: String,
}

impl Required for SearchPhotos {
    fn get_route(&self) -> &'static str {
        routes::SEARCH_PHOTOS
    }
    fn to_query(&self) -> String {
        const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"');
        let query = utf8_percent_encode(&self.query, FRAGMENT).to_string();
        format!("query={}", query)
    }
}

#[derive(Deserialize, Debug)]
pub struct Optionals {
    page: Option<u32>,
    per_page: Option<u32>,
    collections: Option<String>,
    orientation: Option<Orientation>,
}

impl Optionals {
    fn array_to_string(arr: &Vec<u32>) -> String {
        let mut query: String = arr.iter().map(|item| format!("{},", item.to_string())).collect();
        query.pop();
        query
    }
    fn page(&self) -> String {
        match self.page {
            Some(page) => format!("&page={}", page),
            _ => String::from("")
        }
    }
    fn per_page(&self) -> String {
        match self.per_page {
            Some(per_page) => format!("&per_page={}", per_page),
            _ => String::from("")
        }
    }
    fn collections(&self) -> String {
        match &self.collections {
            Some(collections) => format!("&collections={}", collections),
            _ => String::from("")
        }
    }
}

impl Optional for Optionals {
    fn to_query(&self, path: &str) -> String {
        let mut query = String::from("");
        match path {
            routes::SEARCH_PHOTOS => {
                query = format!("{qu}{param}", qu = query, param = self.page());
                query = format!("{qu}{param}", qu = query, param = self.per_page());
                query = format!("{qu}{param}", qu = query, param = self.collections());
                query
            }
            _ => query
        }
    }
}

#[derive(Deserialize, Debug)]
enum Orientation {
    Landscape,
    Portrait,
    Squarish,
}

impl fmt::Display for Orientation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Orientation::Landscape => write!(f, "{}", "landscape"),
            Orientation::Portrait => write!(f, "{}", "portrait"),
            Orientation::Squarish => write!(f, "{}", "squarish"),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn array_to_string() {
        assert_eq!(Optionals::array_to_string(&vec![196,197]),"196,197");
        assert_eq!(Optionals::array_to_string(&vec![196]),"196");
        assert_eq!(Optionals::array_to_string(&Vec::<u32>::new()),"");
    }
}