#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::Response;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};

fn main() {
    println!("Hello Phoenix");

    rocket::ignite()
        .attach(Cors {})
        .launch();
}

/// `Cors` middleware which implement [Fairing](https://rocket.rs/v0.4/guide/fairings/)
/// so it can be `attach`(ed) before the rocket launching.
struct Cors {}

impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "CORS Header",
            kind: Kind::Response,
        }
    }

    fn on_response(&self, _request: &Request, response: &mut Response) {
        response.adjoin_raw_header("Access-Control-Allow-Methods", "PUT, POST, GET, OPTIONS, REMOVE");
        response.adjoin_raw_header("Access-Control-Allow-Origin", "*");
        response.adjoin_raw_header("Access-Control-Allow-Credentials", "true");
        response.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type");
    }
}
