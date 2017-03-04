use std::env;
use std::io::{stderr, Write};
use iron::{Iron, Request, Response, IronResult, status};
use router::Router;

mod response;

use super::menu::Menu;
use self::response::APIResponse;

pub fn run_server() {
    let router = router!(index: get "/api/menu/:servery/" => menu_handler);

    let port: u16 = match env::var("PORT") {
        Ok(port_str) => port_str.parse().unwrap(),
        Err(_) => 3000,
    };

    let _server = Iron::new(router).http(("0.0.0.0", port)).unwrap();
    println!("Running on port {}", port);
}

fn menu_handler(request: &mut Request) -> IronResult<Response> {
    match request.extensions.get::<Router>().unwrap().find("servery") {
        Some(servery) => {
            match Menu::fetch() {
                Ok(menu) => {
                    match menu.items_by_servery.get(servery) {
                        Some(items) => Ok(APIResponse::with(status::Ok, APIResponse {
                            payload: items
                        })),
                        None => Ok(Response::with((status::NotFound, "No such servery"))),
                    }
                }
                Err(e) => {
                    writeln!(stderr(), "Unable to fetch menu: {}", e).unwrap();
                    Ok(Response::with((status::InternalServerError, "Unable to fetch menu")))
                }
            }
        }
        None => Ok(Response::with((status::BadRequest, "No servery specified"))),
    }
}