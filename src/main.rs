extern crate iron;

use iron::prelude::*;
use iron::status;

mod http;
use http::router::Router;
mod keeper;
use keeper::registry::Keeper;

fn main() {
    let mut router = Router::new();
    let mut _keeper = Keeper::new();

    router.add_route("subscribe".to_string(), |_: &mut Request| {
        Ok(Response::with((status::Ok, "Hello!")))
    });

    let listen_addr = "localhost:3000";
    println!("Start listening on {}", listen_addr);
    Iron::new(router).http(listen_addr).unwrap();
}
