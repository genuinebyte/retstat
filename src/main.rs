#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use std::io::Cursor;
use rocket::http::Status;
use rocket::{Request, Response};

#[get("/<stat>")]
fn status<'a>(stat: u16) -> Response<'a> {
    let mut resp = Response::new();

    match Status::from_code(stat) {
        Some(status) => resp.set_status(status),
        None => resp.set_status(Status::NotFound)
    }

    resp.set_sized_body(Cursor::new(format!("{}", resp.status().code)));

    resp
}

#[catch(404)]
fn not_found<'a>(req: &Request) -> Response<'a> {
    let mut resp = Response::new();
    resp.set_status(Status::NotFound);
    resp.set_sized_body(Cursor::new("404"));

    resp
}

fn main() {
    rocket::ignite()
        .mount("/", routes![status])
        .register(catchers![not_found])
        .launch();
}
