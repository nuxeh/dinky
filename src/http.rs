use iron::prelude::*;
use iron::mime;
use router::Router;
use params::{Params, Value};
use std::process;
use url::Url;

fn index() -> String {
    String::from("index")
}

fn shorten(link: &str, base: &str) -> String {
    let url = match link.parse::<Url>() {
        Ok(url) => url,
        _ => return String::from(format!("{} '{}'", "invalid URL", link)),
    };




    format!("<a href=\"{}{}\">{}{}</a>", base, link, base, link)
}

fn handle_submission(req: &mut Request) -> IronResult<Response> {
    let cont_html = "text/html".parse::<mime::Mime>().unwrap();

    let mut req_url: Url = req.url.clone().into();
    req_url.set_query(None);
    let req_url = req_url.into_string();

    let client_addr = req.remote_addr;

    let params = req.get_ref::<Params>().unwrap();

    match params.find(&["url"]) {
        Some(&Value::String(ref link)) => {
            info!("submission: <{}> from {}", link, client_addr);
            Ok(Response::with(
                (cont_html, iron::status::Ok, shorten(link, &req_url))
            ))
        },
        _ => Ok(Response::with((iron::status::Ok, index()))),
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
