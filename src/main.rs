extern crate actix_web;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

use actix_web::{server, App, HttpRequest, HttpResponse, fs, http, State, Query, Error, error};
use handlebars::Handlebars;
use std::collections::HashMap;

struct AppState {
    template: Handlebars,
}

fn starting_soon((state, _query): (State<AppState>, Query<HashMap<String, String>>)) -> Result<HttpResponse, Error> {
    let data = serialize_data("starting soon", "main", "startingSoon");
    let html = render_html(state, data)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

fn break_time((state, _query): (State<AppState>, Query<HashMap<String, String>>)) -> Result<HttpResponse, Error> {
    let data = serialize_data("break time", "main", "breakTime");

    let html = render_html(state, data)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
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
        let mut hbs = Handlebars::new();

        register_templates(&mut hbs);

        App::with_state(AppState{template: hbs})
            .handler("/static", fs::StaticFiles::new("./static").unwrap())
            .resource("/", |r| r.method(http::Method::GET).with(starting_soon))
            .resource("/break", |r| r.method(http::Method::GET).with(break_time))
            // .resource("/stream-ending", |r| r.f(stream_ending))
            // .resource("/overlay", |r| r.f(general_overlay))
    })
    .bind("127.0.0.1:3001")
    .expect("Could not initiate web server")
    .run();
}

fn register_templates(hbs: &mut Handlebars) {
    hbs.register_template_string("index", include_str!("../templates/index.hbs")).expect("error registering index");

    hbs.register_template_string("starting-soon", include_str!("../templates/starting-soon.hbs")).expect("error registering starting soon");

    hbs.register_template_string("break-time", include_str!("../templates/break-time.hbs")).expect("error registering starting soon");
}

fn serialize_data(title: &str, css: &str, location: &str) -> serde_json::Value {
    json!({
        "title": title,
        "css": css,
        "location": {
            location: true
        }
    })
}

fn render_html(state: State<AppState>, data: serde_json::Value) -> Result<String, Error> {
    let html = state
        .template
        .render("index", &data)
        .map_err(|_| error::ErrorInternalServerError("Error rendering index"))?;

    Ok(html)
}