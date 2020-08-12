#![feature(proc_macro_hygiene, decl_macro)]

#[macro_use] extern crate rocket;
use rocket::Response;

#[get("/")]
fn get_handler<'a>() -> Response<'a> {
    let mut res = Response::new();
    res.adjoin_raw_header("Access-Control-Allow-Methods", "POST, GET, OPTIONS");
    res.adjoin_raw_header("Access-Control-Allow-Origin", "*");
    res.adjoin_raw_header("Access-Control-Allow-Credentials", "true");
    res.adjoin_raw_header("Access-Control-Allow-Headers", "Content-Type");
    res
}

fn main() {
    println!("Hello Phoenix");
    rocket::ignite().mount("/", routes![get_handler]).launch();
}