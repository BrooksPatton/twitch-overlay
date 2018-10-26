extern crate actix_web;
#[macro_use]
extern crate serde_json;
extern crate handlebars;

use actix_web::{server, App, HttpResponse, fs, http, State, Query, Error, error};
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

fn ending((state, _query): (State<AppState>, Query<HashMap<String, String>>)) -> Result<HttpResponse, Error> {
    let data = serialize_data("stream ending", "main", "ending");
    let html = render_html(state, data)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

fn rust_tutorial((state, _query): (State<AppState>, Query<HashMap<String, String>>)) -> Result<HttpResponse, Error> {
    let data = serialize_data("Introduction to Rust", "rust_tutorial", "rustTutorial");
    let html = render_html(state, data)?;

    Ok(HttpResponse::Ok()
        .content_type("text/html")
        .body(html))
}

fn main() {

    server::new(|| {
        let mut hbs = Handlebars::new();

        register_templates(&mut hbs);

        App::with_state(AppState{template: hbs})
            .handler("/static", fs::StaticFiles::new("./static").unwrap())
            .resource("/", |r| r.method(http::Method::GET).with(starting_soon))
            .resource("/break", |r| r.method(http::Method::GET).with(break_time))
            .resource("/ending", |r| r.method(http::Method::GET).with(ending))
            .resource("/rust-tutorial", |r| r.method(http::Method::GET).with(rust_tutorial))
    })
    .bind("127.0.0.1:3001")
    .expect("Could not initiate web server")
    .run();
}

fn register_templates(hbs: &mut Handlebars) {
    hbs.register_template_string("index", include_str!("../templates/index.hbs")).expect("error registering index");

    hbs.register_template_string("starting-soon", include_str!("../templates/starting-soon.hbs")).expect("error registering starting soon");

    hbs.register_template_string("break-time", include_str!("../templates/break-time.hbs")).expect("error registering break");

    hbs.register_template_string("ending", include_str!("../templates/ending.hbs")).expect("error registering ending");

    hbs.register_template_string("rusttutorial", include_str!("../templates/rusttutorial.hbs")).expect("error registering rusttutorial");
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