extern crate actix_web;

use actix_web::{server, App, HttpRequest, HttpResponse, fs};

fn starting_soon(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body(include_str!("../static/index.html"))
}

fn break_time(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body(include_str!("../static/break_time.html"))
}

fn main() {
    server::new(|| {
        App::new()
            .handler("/static", fs::StaticFiles::new("./static").unwrap())
            .resource("/", |r| r.f(starting_soon))
            .resource("/break", |r| r.f(break_time))
    })
    .bind("127.0.0.1:3001")
    .expect("Could not initiate web server")
    .run();
}
