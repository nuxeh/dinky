use iron::prelude::*;
use iron::mime;
use iron::status::{Found, NotFound, Ok as StatusOk};
use iron::modifiers::Redirect;
use router::Router;
use params::{Params, Value};
use std::process;
use url::Url;
use failure::Error;

use crate::db;
use crate::conf::Conf;

fn index() -> String {
    String::from("index")
}

fn shorten(conf: &Conf, link: &str, base: &str) -> Result<String, Error> {
    if link.parse::<Url>().is_err() {
        bail!("{} '{}'", "invalid URL", link);
    };
    let hash = db::insert_url(conf, link)?;
    Ok(format!("<a href=\"{}{}\">{}{}</a>", base, hash, base, hash))
}

fn submit(conf: &Conf, req: &mut Request) -> IronResult<Response> {
    let html = "text/html".parse::<mime::Mime>().unwrap();

    let mut req_url: Url = req.url.clone().into();
    req_url.set_query(None);
    let req_url = req_url.into_string();

    let client_addr = req.remote_addr;
    let params = req.get_ref::<Params>().unwrap();

    match params.find(&["url"]) {
        Some(&Value::String(ref link)) => {
            info!("submission <{}> from {}", link, client_addr);
            match shorten(conf, link, &req_url) {
                Ok(l) => Ok(Response::with((html, StatusOk, l))),
                Err(e) => Ok(Response::with((StatusOk, format!("{}", e)))),
            }
        },
        _ => Ok(Response::with((StatusOk, index()))),
    }
}

fn redirect(conf: &Conf, req: &mut Request) -> IronResult<Response> {
    let query = &req.extensions
        .get::<Router>()
        .unwrap()
        .find("hash")
        .unwrap_or("/");

    match db::get_url(conf, query) {
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
    Ok(Response::with(NotFound))
}

pub fn listen(conf: &'static Conf) {

    let bind = format!("{}:{}", conf.settings.bind, conf.settings.port);

    // FnOnce + static requirement - no borrow, only move
    let router = router!{
        submit: get "/" => move |request: &mut Request| submit(conf, request),
        redirect: get "/:hash" => move |request: &mut Request| redirect(conf, request),
        favicon: get "/favicon.ico" => not_found,
    };

    info!("dinky starting on {}", bind);

    Iron::new(router).http(bind).unwrap_or_else(|err| {
        error!("starting server: {}", err);
        process::exit(1);
    });
}
