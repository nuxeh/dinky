use failure::Error;
use iron::mime;
use iron::modifiers::Redirect;
use iron::prelude::*;
use iron::status::{Found, NotFound, Ok as StatusOk};
use params::{Params, Value};
use router::Router;
use std::fs::read_to_string;
use std::process;
use url::Url;

use crate::conf::Conf;
use crate::db;

// pull in static index content
include!(concat!(env!("OUT_DIR"), "/index.rs"));

fn index(conf: &Conf, content: &str) -> String {
    let page = if let Some(t) = &conf.index.html {
        read_to_string(t).ok()
    } else {
        None
    };

    let page = page
        .unwrap_or(String::from(DEFAULT_INDEX))
        .replace("{{ver}}", CRATE_VERSION)
        .replace("{{content}}", content);

    page
}

fn form(conf: &Conf) -> String {
    let form = if let Some(t) = &conf.index.form {
        read_to_string(t).ok()
    } else {
        None
    };

    form.unwrap_or(String::from(DEFAULT_FORM))
}

fn css(conf: &Conf) -> IronResult<Response> {
    let css = if let Some(c) = &conf.index.css {
        read_to_string(c).ok()
    } else {
        None
    };

    let css = if let Some(c) = &css {
        c.as_str()
    } else {
        DEFAULT_CSS
    };

    Ok(Response::with((StatusOk, css)))
}

fn err<S>(text: S) -> String
where
    S: std::fmt::Display
{
    format!("<div id=\"dinky-error\">{}</div>", text)
}

fn shorten(conf: &Conf, link: &str, base: &str) -> Result<String, Error> {
    if link.parse::<Url>().is_err() {
        bail!("invalid URL \"{}\"", link);
    };
    let hash = db::insert_url(conf, link)?;
    let url = format!("{}{}", base, hash);
    Ok(format!(r#"<a href="{}" id="dinky-link">{}</a>"#, url, url))
}

fn submit(conf: &Conf, req: &mut Request) -> IronResult<Response> {
    let html = "text/html".parse::<mime::Mime>().unwrap();

    let req_url = if conf.settings.base_url.is_empty() {
        let mut req_url: Url = req.url.clone().into();
        req_url.set_query(None);
        req_url.into_string()
    } else {
        conf.settings.base_url.clone()
    };

    let client_addr = req.remote_addr;
    let params = req.get_ref::<Params>().unwrap();

    match params.find(&["url"]) {
        Some(&Value::String(ref link)) => {
            info!("submission <{}> from {}", link, client_addr);
            let resp = match shorten(conf, link, &req_url) {
                Ok(l) => index(&conf, &l),
                Err(e) => index(&conf, &err(e)),
            };
            Ok(Response::with((html, StatusOk, resp)))
        },
        _ => {
            if conf.index.hidden {
                Ok(Response::with(NotFound))
            } else {
                Ok(Response::with((html, StatusOk, index(&conf, &form(conf)))))
            }
        }
    }
}

fn redirect(conf: &Conf, req: &mut Request) -> IronResult<Response> {
    let html = "text/html".parse::<mime::Mime>().unwrap();

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
            Ok(Response::with((html, StatusOk, index(conf, &err("Link not found!")))))
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
        css: get "/dinky.css" => move |_: &mut Request| css(conf),
        favicon: get "/favicon.ico" => not_found,
    };

    info!("dinky starting on {}", bind);

    Iron::new(router).http(bind).unwrap_or_else(|err| {
        error!("starting server: {}", err);
        process::exit(1);
    });
}
