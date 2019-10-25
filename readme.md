Rust Library to work with the Unsplash API.

`main.rs` provides an example using Actix.
`lib.rs` will eventually become its own crate.

Notes:
Ideally, this library would use a single `reqwest::r#async::Client` and reuse it for every request, but this might not be possible with Actix anymore. You used to be able to return a Future with an Item equal to something other than an `actix::HttpRequest` (ie, String or whatever), but it looks like that is no longer the case.
