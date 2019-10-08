use iron::prelude::*;
use iron::mime;
use iron::status::{Found, NotFound, Ok as StatusOk};
use iron::modifiers::Redirect;
use router::Router;
use params::{Params, Value};
use std::process;
use url::Url;
use failure::Error;
use crate::db::{insert_url, get_url};

fn index() -> String {
    String::from("index")
}

fn shorten(link: &str, base: &str) -> String {
    let url = match link.parse::<Url>() {
        Ok(url) => url,
        _ => return String::from(format!("{} '{}'", "invalid URL", link)),
    };

    let hash = match insert_url(link) {
        Ok(h) => h,
        Err(e) => {
            error!("adding URL to database: {}", e); "".to_string()
        },
    };

    format!("<a href=\"{}{}\">{}{}</a>", base, hash, base, hash)
}

fn submit(req: &mut Request) -> IronResult<Response> {
    let html = "text/html".parse::<mime::Mime>().unwrap();

    let mut req_url: Url = req.url.clone().into();
    req_url.set_query(None);
    let req_url = req_url.into_string();

    let client_addr = req.remote_addr;
    let params = req.get_ref::<Params>().unwrap();

    match params.find(&["url"]) {
        Some(&Value::String(ref link)) => {
            info!("submission: <{}> from {}", link, client_addr);
            Ok(Response::with((html, StatusOk, shorten(link, &req_url))))
        },
        _ => Ok(Response::with((StatusOk, index()))),
    }
}

fn redirect(req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
        .get::<Router>()
        .unwrap()
        .find("hash")
        .unwrap_or("/");

    match get_url(query) {
        Ok(l) => {
            let url = iron::Url::parse(&l).unwrap();
            Ok(Response::with((Found, Redirect(url))))
        },
        Err(e) => {
            warn!("{}", e);
            Ok(Response::with((StatusOk, "Link not found!")))
        },
    }
}

fn not_found(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((NotFound)))
}

pub fn listen() {
    let router = router!{
        submit: get "/" => submit,
        favicon: get "/favicon.ico" => not_found,
        redirect: get "/:hash" => redirect,
    };

    Iron::new(router).http("127.0.0.1:3000").unwrap_or_else(|err| {
        error!("error starting server: {}", err);
        process::exit(1);
    });
}
