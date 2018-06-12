extern crate actix_web;

use actix_web::{App, server, fs};

fn main() {
    server::new(|| {
        App::new()
            .handler("/", fs::StaticFiles::new("./dist/"))
    }).bind("127.0.0.1:8080")
        .unwrap()
        .run();
}
