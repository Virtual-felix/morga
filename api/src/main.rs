#![feature(proc_macro_hygiene, decl_macro)]

mod controller;
mod model;
mod service;

use rocket::{
    fairing::{Fairing, Info, Kind},
    Request, Response,
};
use std::sync::Mutex;

use service::quest;

fn main() {
    println!("Hello Phoenix");

    // NOTE: Services should be stateless so no one one their method would need to be mutable.
    //       Mutex is used so the managed state of rocket implement Sync + Send.
    //       https://rocket.rs/v0.4/guide/state/#managed-state
    //       https://github.com/SergioBenitez/Rocket/issues/359
    let quest_service = Mutex::new(quest::new());

    let routes = rocket::routes![
        controller::quest::create,
        controller::quest::get,
        controller::quest::list,
        controller::quest::update,
        controller::quest::delete,
    ];

    rocket::ignite()
        .manage(quest_service)
        .mount("/", routes)
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
