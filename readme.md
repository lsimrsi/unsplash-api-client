# Unsplash API
Rust Library to work with the Unsplash API.

`main.rs` provides an example using Actix (starts the server).

`lib.rs` will eventually become its own crate.

## Usage
Only works with curl for now. There is a `curl.txt` file with curl commands you can run.

## Notes
I'm using a `reqwest::Client` (not async) in an `Arc`, so I think at least the same thread pool will be used for the `Client` for all requests. This was recommended in the `reqwest` docs:
https://docs.rs/reqwest/0.9.22/reqwest/struct.Client.html:
>The Client holds a connection pool internally, so it is advised that you create one and reuse it.

I think (and I could be wrong) that to avoid blocking, this library should be using `reqwest::r#async::Client`, but then Rust will throw an error about `AsyncFactory` not being implemented for the `to_async` handler.

In order to avoid that error I'm using `reqwest::Client` in conjunction with `actix_web::web::block` in the `to_async` handler.

## License
[MIT](https://choosealicense.com/licenses/mit/)
