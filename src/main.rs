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

fn stream_ending(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body(include_str!("../static/thanks_for_watching.html"))
}

fn general_overlay(_req: &HttpRequest) -> HttpResponse {
    HttpResponse::Ok()
        .body(include_str!("../static/general_overlay.html"))
}

fn main() {
    server::new(|| {
        App::new()
            .handler("/static", fs::StaticFiles::new("./static").unwrap())
            .resource("/", |r| r.f(starting_soon))
            .resource("/break", |r| r.f(break_time))
            .resource("/stream-ending", |r| r.f(stream_ending))
            .resource("/overlay", |r| r.f(general_overlay))
    })
    .bind("127.0.0.1:3001")
    .expect("Could not initiate web server")
    .run();
}
