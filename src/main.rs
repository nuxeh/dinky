/*
 * dinky
 *
 * URL shortening service.
 *
 */

extern crate docopt;
#[macro_use]
extern crate log;
extern crate atty;
extern crate stderrlog;
#[macro_use]
extern crate serde_derive;
extern crate iron;
#[macro_use]
extern crate router;
extern crate params;
extern crate url;
extern crate directories;
#[macro_use]
extern crate diesel;

use atty::{is, Stream};
use docopt::Docopt;
use std::path::{Path, PathBuf};
use stderrlog::{ColorChoice, Timestamp};
use directories::BaseDirs;

mod conf;
mod db;
mod db_schema;
mod db_models;
mod hash;
mod http;

const USAGE: &str = "
Link shortening service.

Usage:
    dinky [options] [-v...] [--db=PATH]

Options:
    -h --help       Show this help message.
    --version       Print version.
    -v --verbose    Show extra information.
    -d --db=PATH    Use a sqlite database at PATH.
    -c --conf=PATH  Use configuration file at PATH.
    -t --timestamp  Force timestamps.
";

#[derive(Debug, Deserialize, Default)]
pub struct Args {
    flag_verbose: usize,
    flag_db: Option<PathBuf>,
    flag_conf: Option<PathBuf>,
    flag_timestamp: bool,
}

const MIN_VERBOSITY: usize = 2;

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some("0.1.0".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    // don't output colours or include timestamps on stderr if piped
    let (coloured_output, mut timestamp) = if is(Stream::Stderr) {
        (ColorChoice::Auto, Timestamp::Second)
    } else {
        (ColorChoice::Never, Timestamp::Off)
    };

    // force timestamp
    if args.flag_timestamp {
        timestamp = Timestamp::Second;
    };

    // start logger
    stderrlog::new()
        .module(module_path!())
        .modules(vec![
            "http",
        ])
        .verbosity(args.flag_verbose + MIN_VERBOSITY)
        .timestamp(timestamp)
        .color(coloured_output)
        .init()
        .unwrap();

    info!("dinky starting...");
    http::listen();
}

fn expand_tilde(path: &Path) -> PathBuf {
    match (BaseDirs::new(), path.strip_prefix("~")) {
        (Some(bd), Ok(stripped)) => bd.home_dir().join(stripped),
        _ => path.to_owned(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_expand_tilde() {
        let homedir: PathBuf = BaseDirs::new()
            .unwrap()
            .home_dir()
            .to_owned();

        assert_eq!(expand_tilde(&PathBuf::from("/")),
            PathBuf::from("/"));
        assert_eq!(expand_tilde(&PathBuf::from("/abc/~def/ghi/")),
            PathBuf::from("/abc/~def/ghi/"));
        assert_eq!(expand_tilde(&PathBuf::from("~/")),
            PathBuf::from(format!("{}/", homedir.to_str().unwrap())));
        assert_eq!(expand_tilde(&PathBuf::from("~/ac/df/gi/")),
            PathBuf::from(format!("{}/ac/df/gi/", homedir.to_str().unwrap())));
    }
}
