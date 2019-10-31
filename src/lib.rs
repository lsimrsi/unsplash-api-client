use percent_encoding::{utf8_percent_encode, AsciiSet, CONTROLS};
use reqwest;
use serde::Deserialize;
use std::fmt;
use std::sync::Arc;
use std::sync::atomic::{AtomicUsize, Ordering};
// use std::ops::{Deref, DerefMut};

pub mod routes {
    pub enum Method {
        Get,
        Post,
    }

    pub struct Route {
        pub method: Method,
        pub path: &'static str,
    }

    impl Route {
        pub fn new(method: Method, path: &'static str) -> Route {
            Route { method, path }
        }
    }

    pub const BASE_URL: &'static str = "https://api.unsplash.com/";
    pub const SEARCH_PHOTOS: &'static str = "search/photos";
    pub const PHOTOS_RANDOM: &'static str = "photos/random";
}

#[derive(Clone)]
pub struct Unsplash {
    client: Arc<reqwest::Client>,
    rate_limit: Arc<AtomicUsize>,
    rate_remaining: Arc<AtomicUsize>,
    access_key: String,
    _secret_key: String,
}

impl Unsplash {
    pub fn new(access_key: &str, secret_key: &str) -> Unsplash {
        Unsplash {
            client: Arc::new(reqwest::Client::new()),
            access_key: access_key.to_owned(),
            _secret_key: secret_key.to_owned(),
            rate_limit: Arc::new(AtomicUsize::new(0)),
            rate_remaining: Arc::new(AtomicUsize::new(50)),
        }
    }

    pub fn passthrough(&self, path_and_query: &str, method: &str) -> reqwest::Result<String> {
        let url = format!("{}{}", routes::BASE_URL, path_and_query);
        let mut res: reqwest::Response;
        match method {
            "GET" => res = self.client.get(&url).send()?,
            "POST" => res = self.client.post(&url).send()?,
            _ => return Ok("unknown request method".to_string())
        }

        if let Some(limit) = res.headers().get("X-Ratelimit-Limit") {
            if let Ok(val) = limit.to_str() {
                let num = val.parse().expect("couldn't parse header: X-Ratelimit-Limit");
                self.rate_limit.store(num, Ordering::Relaxed);
                println!("X-Ratelimit-Limit: {}", num);
            }
        }

        if let Some(remaining) = res.headers().get("X-Ratelimit-Remaining") {
            if let Ok(val) = remaining.to_str() {
                let num = val.parse().expect("couldn't parse header: X-Ratelimit-Remaining");
                self.rate_limit.store(num, Ordering::Relaxed);
                println!("X-Ratelimit-Remaining: {}", num);
            }
        }

        res.text()
    }

    pub fn send<R, O>(&self, required: R, optional: O) -> reqwest::Result<String>
    where
        R: Required,
        O: Optional,
    {
        let url = format!(
            "{base}{path}?{required}{optional}{key}",
            base = routes::BASE_URL,
            path = required.get_route().path,
            required = required.to_query(),
            optional = optional.to_query(required.get_route().path),
            key = self.get_access_key_param()
        );

        println!("url: {}", &url);
        let mut res: reqwest::Response;
        match required.get_route().method {
            routes::Method::Get => res = self.client.get(&url).send()?,
            routes::Method::Post => res = self.client.post(&url).send()?,
        }

        if let Some(limit) = res.headers().get("X-Ratelimit-Limit") {
            if let Ok(val) = limit.to_str() {
                let num = val.parse().expect("couldn't parse header: X-Ratelimit-Limit");
                self.rate_limit.store(num, Ordering::Relaxed);
                println!("X-Ratelimit-Limit: {}", num);
            }
        }

        if let Some(remaining) = res.headers().get("X-Ratelimit-Remaining") {
            if let Ok(val) = remaining.to_str() {
                let num = val.parse().expect("couldn't parse header: X-Ratelimit-Remaining");
                self.rate_limit.store(num, Ordering::Relaxed);
                println!("X-Ratelimit-Remaining: {}", num);
            }
        }

        res.text()
    }

    fn get_access_key_param(&self) -> String {
        format!("&client_id={}", self.access_key)
    }
}

pub trait Required {
    fn get_route(&self) -> routes::Route;
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
    fn get_route(&self) -> routes::Route {
        routes::Route::new(routes::Method::Get, routes::SEARCH_PHOTOS)
    }

    fn to_query(&self) -> String {
        const FRAGMENT: &AsciiSet = &CONTROLS.add(b' ').add(b'"');
        let query = utf8_percent_encode(&self.query, FRAGMENT).to_string();
        format!("query={}", query)
    }
}

#[derive(Deserialize, Debug)]
pub struct PhotosRandom {}

impl Required for PhotosRandom {
    fn get_route(&self) -> routes::Route {
        routes::Route::new(routes::Method::Get, routes::PHOTOS_RANDOM)
    }
    fn to_query(&self) -> String {
        String::from("")
    }
}

#[derive(Deserialize, Debug)]
pub struct Optionals {
    page: Option<u32>,
    per_page: Option<u32>,
    collections: Option<String>,
    orientation: Option<Orientation>,
    featured: Option<bool>,
    username: Option<String>,
    query: Option<String>,
    count: Option<u8>
}

impl Optionals {
    fn _array_to_string(arr: &Vec<u32>) -> String {
        let mut query: String = arr
            .iter()
            .map(|item| format!("{},", item.to_string()))
            .collect();
        query.pop();
        query
    }
    fn page(&self) -> String {
        match self.page {
            Some(page) => format!("&page={}", page),
            _ => String::from(""),
        }
    }
    fn per_page(&self) -> String {
        match self.per_page {
            Some(per_page) => format!("&per_page={}", per_page),
            _ => String::from(""),
        }
    }
    fn collections(&self) -> String {
        match &self.collections {
            Some(collections) => format!("&collections={}", collections),
            _ => String::from(""),
        }
    }
    fn orientation(&self) -> String {
        match &self.orientation {
            Some(orientation) => format!("&orientation={}", orientation),
            _ => String::from(""),
        }
    }
    fn featured(&self) -> String {
        match self.featured {
            Some(featured) => format!("&featured={}", featured),
            _ => String::from(""),
        }
    }
    fn username(&self) -> String {
        match &self.username {
            Some(username) => format!("&username={}", username),
            _ => String::from(""),
        }
    }
    fn query(&self) -> String {
        match &self.query {
            Some(query) => format!("&query={}", query),
            _ => String::from(""),
        }
    }
    fn count(&self) -> String {
        match &self.count {
            Some(count) => format!("&count={}", count),
            _ => String::from(""),
        }
    }
}

impl Optional for Optionals {
    fn to_query(&self, path: &str) -> String {
        let mut qs = String::from("");

        match path {
            routes::SEARCH_PHOTOS => {
                qs = format!("{}{}", qs, self.page());
                qs = format!("{}{}", qs, self.per_page());
                qs = format!("{}{}", qs, self.collections());
                qs = format!("{}{}", qs, self.orientation());
                qs
            }

            routes::PHOTOS_RANDOM => {
                // match &self.collections {
                //     Some(_) => qs = format!("{}{}", qs, self.collections()),
                //     _ => qs = format!("{}{}", qs, self.query()),
                // };
                qs = format!("{}{}", qs, self.collections());
                qs = format!("{}{}", qs, self.query());
                qs = format!("{}{}", qs, self.featured());
                qs = format!("{}{}", qs, self.username());
                qs = format!("{}{}", qs, self.orientation());
                qs = format!("{}{}", qs, self.count());
                qs
            }

            _ => qs,
        }
    }
}

#[derive(Deserialize, Debug)]
enum Orientation {
    #[serde(rename = "landscape")]
    Landscape,
    #[serde(rename = "portrait")]
    Portrait,
    #[serde(rename = "squarish")]
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
        assert_eq!(Optionals::_array_to_string(&vec![196, 197]), "196,197");
        assert_eq!(Optionals::_array_to_string(&vec![196]), "196");
        assert_eq!(Optionals::_array_to_string(&Vec::<u32>::new()), "");
    }
}
