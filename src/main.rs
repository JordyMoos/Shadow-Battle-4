extern crate actix_web;

use actix_web::{http, App, HttpRequest, server};
use std::cell::Cell;

struct AppState {
    counter: Cell<usize>,
}

fn index(req: HttpRequest<AppState>) -> String {
    let count = req.state().counter.get() + 1;
    req.state().counter.set(count);

    format!("Request number: {}", count)
}

fn main() {
    server::new(|| {
        App::with_state(AppState { counter: Cell::new(0) })
            .resource("/", |r| r.method(http::Method::GET).f(index))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
