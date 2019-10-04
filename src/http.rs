use iron::prelude::*;
use iron::mime;
use router::Router;
use params::{Params, Value};
use std::process;
use url::Url;

fn index() -> String {
    String::from("index")
}

fn process_link(link: &str) -> String {
    let url = match link.parse::<Url>() {
        Ok(url) => url,
        _ => return String::from("invalid URL"),
    };

    info!("adding link <{}>", link);

    format!("<a href=\"{}\">{}</a>", link, link)
}

fn handle_submission(req: &mut Request) -> IronResult<Response> {
    let map = req.get_ref::<Params>().unwrap();
    let content_type = "text/html".parse::<mime::Mime>().unwrap();

    info!("{:?}", map);

    //let content_type = mime::APPLICATION_JSON;

    match map.find(&["url"]) {
        Some(&Value::String(ref name)) => {
            Ok(Response::with(
                (content_type, iron::status::Ok, process_link(name))
            ))
        },
        _ => {
            Ok(Response::with(
                (iron::status::Ok, index())
            ))
        },
    }
}

fn handle_redirect(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions.get::<Router>()
        .unwrap().find("hash").unwrap_or("/");
    Ok(Response::with((iron::status::Ok, *query)))
}

pub fn listen() {
    let router = router!{
        submit: get "/" => handle_submission,
        redirect: get "/:hash" => handle_redirect,
    };

    Iron::new(router).http("127.0.0.1:3000").unwrap_or_else(|err| {
        error!("error starting server: {}", err);
        process::exit(1);
    });
}
