use iron::prelude::*;
use iron::mime;
use iron::status::{Found, NotFound, Ok as StatusOk};
use iron::modifiers::Redirect;
use router::Router;
use params::{Params, Value};
use std::process;
use std::sync::Arc;
use url::Url;

use crate::db::{insert_url, get_url};
use crate::conf::Conf;

fn index() -> String {
    String::from("index")
}

fn shorten(conf: Arc<&Conf>, link: &str, base: &str) -> String {
    if link.parse::<Url>().is_err() {
        return String::from(format!("{} '{}'", "invalid URL", link));
    };

    let hash = match insert_url(conf, link) {
        Ok(h) => h,
        Err(e) => {
            error!("adding URL to database: {}", e); "".to_string();
            return "Database error.".to_string();
        },
    };

    format!("<a href=\"{}{}\">{}{}</a>", base, hash, base, hash)
}

fn submit(conf: Arc<&Conf>, req: &mut Request) -> IronResult<Response> {
    let html = "text/html".parse::<mime::Mime>().unwrap();

    let mut req_url: Url = req.url.clone().into();
    req_url.set_query(None);
    let req_url = req_url.into_string();

    let client_addr = req.remote_addr;
    let params = req.get_ref::<Params>().unwrap();

    match params.find(&["url"]) {
        Some(&Value::String(ref link)) => {
            info!("submission: <{}> from {}", link, client_addr);
            Ok(Response::with((html, StatusOk, shorten(conf, link, &req_url))))
        },
        _ => Ok(Response::with((StatusOk, index()))),
    }
}

fn redirect(conf: Arc<&Conf>, req: &mut Request) -> IronResult<Response> {
    let ref query = req.extensions
        .get::<Router>()
        .unwrap()
        .find("hash")
        .unwrap_or("/");

    match get_url(conf, query) {
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

pub fn listen(conf: Arc<&'static Conf>) {

    let bind = format!("{}:{}", conf.settings.bind, conf.settings.port);

    let mut router = Router::new();
    let a = conf.clone();
    // FnOnce + static requirement - no borrow, only move
    router.get("/", move |request: &mut Request| submit(a.clone(), request), "submit");
    /*
    let router = router!{
        submit: get "/" => move |request: &mut Request| submit(Arc::clone(&conf), request),
        redirect: get "/:hash" => move |request: &mut Request| redirect(Arc::clone(&conf), request),
        favicon: get "/favicon.ico" => not_found,
    };
    */

    info!("dinky starting on {}", bind);

    Iron::new(router).http(bind).unwrap_or_else(|err| {
        error!("starting server: {}", err);
        process::exit(1);
    });
}
