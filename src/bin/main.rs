use actix_files as fs;
use actix_web::{guard, http, middleware, web, App, Error, HttpRequest, HttpResponse, HttpServer};
use futures::Future;
use std::env;
use unsplash_api::{self, routes, Unsplash};

fn unsplash_get(
    req: HttpRequest,
    unsplash: web::Data<Unsplash>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    let mut path_and_query = match req.uri().path_and_query() {
        Some(paq) => paq.to_string(),
        None => "".to_string(),
    };
    path_and_query.drain(0..10); // removes /unsplash/ from beginning of String

    actix_web::web::block(move || unsplash.passthrough_get(&path_and_query))
        .from_err()
        .and_then(|body_text| {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(body_text)
        })
}

fn search_photos(
    required: web::Query<unsplash_api::SearchPhotos>,
    optional: web::Query<unsplash_api::Optionals>,
    unsplash: web::Data<Unsplash>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    send_request(required, optional, unsplash)
}

fn limit_info(
    unsplash: web::Data<Unsplash>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
    actix_web::web::block(move || unsplash.get_limit_info())
        .from_err()
        .and_then(|res| {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)
        })
}

// fn photos_random(
//     required: web::Query<unsplash_api::PhotosRandom>,
//     optional: web::Query<unsplash_api::Optionals>,
//     unsplash: web::Data<Unsplash>,
// ) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {
//     send_request(required, optional, unsplash)
// }

fn send_request<R>(
    required: web::Query<R>,
    optional: web::Query<unsplash_api::Optionals>,
    unsplash: web::Data<Unsplash>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error>
where
    R: unsplash_api::Required + Send + 'static,
{
    actix_web::web::block(move || unsplash.send(required.into_inner(), optional.into_inner()))
        .from_err()
        .and_then(|res| {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)
        })
}

fn p404() -> Result<fs::NamedFile, Error> {
    Ok(fs::NamedFile::open("static/404.html")?.set_status_code(http::StatusCode::NOT_FOUND))
}

// if port is defined as an environment variable, use that instead
// for example, Heroku defines its own port
fn get_server_port() -> u16 {
    env::var("PORT")
        .unwrap_or_else(|_| 5000.to_string())
        .parse()
        .expect("PORT must be a number")
}

fn main() {
    let access_key: String = env::var("UNSPLASH_ACCESS_KEY").unwrap();
    let secret_key: String = env::var("UNSPLASH_SECRET_KEY").unwrap();
    let unsplash = Unsplash::new(&access_key, &secret_key);

    HttpServer::new(move || {
        App::new()
            .data(unsplash.clone())
            .wrap(middleware::Logger::default())
            .service(
                web::scope("/unsplash")
                    .default_service(web::get().to_async(unsplash_get))
                    .route(routes::SEARCH_PHOTOS, web::get().to_async(search_photos))
                    .route(routes::LIMIT_INFO, web::get().to_async(limit_info)), // .route(routes::PHOTOS_RANDOM, web::get().to_async(photos_random))
            )
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
