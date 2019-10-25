use actix_files as fs;
use actix_web::{http, middleware, web, guard, App, Error, HttpResponse, HttpServer};
use futures::Future;
use reqwest::{self};
use std::env;
use unsplash_api::{self, Unsplash, routes};

#[macro_use]
extern crate lazy_static;

fn search_photos(
    unsplash: web::Data<Unsplash>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let builder = unsplash.search_photos("fish");
    actix_web::web::block(move || builder.send())
        .from_err()
        .and_then(|mut res| {
            let body = match res.text() {
                Ok(body) => body,
                Err(error) => {
                    println!("get text error: {}", error);
                    "{{\"error\": \"Error getting response text.\"}}".to_string()
                }
            };
            match res.status() {
                reqwest::StatusCode::OK => HttpResponse::Ok()
                    .content_type("application/json")
                    .body(body),
                _ => HttpResponse::InternalServerError()
                    .content_type("application/json")
                    .body(body),
            }
        })
}

fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(http::StatusCode::NOT_FOUND))
}

// if port is defined as an environment variable, use that instead
// for example, Heroku defines its own port
fn get_server_port() -> u16 {
    env::var("PORT")
        .unwrap_or_else(|_| "5000".to_string())
        .parse()
        .expect("PORT must be a number")
}

lazy_static! {
    static ref ACCESS_KEY: String = env::var("ACCESS_KEY").unwrap();
    static ref SECRET_KEY: String = env::var("SECRET_KEY").unwrap();
}

fn main() {
    HttpServer::new(move || {
        App::new()
            .data(Unsplash::new(
                &ACCESS_KEY,
                &SECRET_KEY,
            ))
            .wrap(middleware::Logger::default())
            .service(web::resource(routes::SEARCH_PHOTOS).route(web::get().to_async(search_photos)))
            .service(fs::Files::new("/", "static/build").index_file("index.html"))
            .default_service(
                // 404 for GET request
                web::resource("")
                    .route(web::get().to(p404))
                    // all requests that are not GET
                    .route(
                        web::route()
                            .guard(guard::Not(guard::Get()))
                            .to(HttpResponse::MethodNotAllowed),
                    ),
            )
    })
    .bind(("0.0.0.0", get_server_port()))
    .unwrap()
    .run()
    .unwrap();
}
