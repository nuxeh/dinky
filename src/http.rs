use iron::prelude::*;
use router::Router;
use params::{Params, Value};
use std::process;

fn handle_submission(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();

    info!("{:?}", map);

    match map.find(&["url"]) {
        Some(&Value::String(ref name)) if name == "Marie" => {
            Ok(Response::with(
                (iron::status::Ok, "Welcome back, Marie!")
            ))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}

fn handle_redirect(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>()
        .unwrap().find("hash").unwrap_or("/");
    Ok(Response::with((iron::status::Ok, *query)))
}

pub fn run() {
    let router = router!{
        submit: get "/" => handle_submission,
        redirect: get "/:hash" => handle_redirect,
    };

    Iron::new(router).http("localhost:3000").unwrap_or_else(|err| {
        error!("error starting server: {}", err);
        process::exit(1);
    });
}
