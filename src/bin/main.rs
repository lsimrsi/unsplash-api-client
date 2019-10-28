use actix_files as fs;
use actix_web::{guard, http, middleware, web, App, Error, HttpResponse, HttpServer};
use futures::Future;
use std::env;
use unsplash_api::{self, routes, Unsplash};

fn search_photos(
    required: web::Query<unsplash_api::SearchPhotos>,
    optional: web::Query<unsplash_api::Optionals>,
    unsplash: web::Data<Unsplash>,
) -> impl Future<Item = HttpResponse, Error = actix_web::Error> {

    println!("{:?}", optional);
    actix_web::web::block(move || unsplash.get(required.into_inner(), optional.into_inner()))
        .from_err()
        .and_then(|res| {
            HttpResponse::Ok()
                .content_type("application/json")
                .body(res)

            // let body = match res.text() {
            //     Ok(body) => body,
            //     Err(error) => {
            //         println!("get text error: {}", error);
            //         "{{\"error\": \"Error getting response text.\"}}".to_string()
            //     }
            // };
            // match res.status() {
            //     reqwest::StatusCode::OK => 
            //     _ => HttpResponse::InternalServerError()
            //         .content_type("application/json")
            //         .body(body),
            // }
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

    HttpServer::new(move || {
        App::new()
            .data(Unsplash::new(&access_key, &secret_key))
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
