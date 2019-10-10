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
extern crate time;
#[macro_use]
extern crate failure;

mod conf;
mod db;
mod db_schema;
mod db_models;
mod hash;
mod http;

use atty::{is, Stream};
use docopt::Docopt;
use std::process;
use std::path::{Path, PathBuf};
use stderrlog::{ColorChoice, Timestamp};
use directories::{ProjectDirs, BaseDirs};
use failure::Error;
use std::fs;
use conf::Conf;
use std::sync::Arc;

const USAGE: &str = "
Link shortening service.

Usage:
    dinky [options] [-v...] [--db=PATH]

Options:
    -h --help       Show this help message.
    --version       Print version.
    -v --verbose    Show extra information.
    -c --conf=PATH  Use configuration file at PATH.
    -t --timestamp  Force timestamps.
";

#[derive(Debug, Deserialize, Default)]
pub struct Args {
    flag_verbose: usize,
    flag_conf: Option<PathBuf>,
    flag_timestamp: bool,
}

const MIN_VERBOSITY: usize = 2;

fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.version(Some("0.1.0".to_string())).deserialize())
        .unwrap_or_else(|e| e.exit());

    let (coloured_output, mut timestamp) = if is(Stream::Stderr) {
        (ColorChoice::Auto, Timestamp::Second)
    } else {
        (ColorChoice::Never, Timestamp::Off)
    };

    if args.flag_timestamp {
        timestamp = Timestamp::Second;
    };

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

    let dirs = ProjectDirs::from("org", "", "dinky").unwrap();

    let conf_path = match args.flag_conf {
        Some(ref p) => expand_tilde(p),
        None => dirs.config_dir().join("config.toml")
    };

    info!("using configuration at '{}'", conf_path.display());
    init_conf(&conf_path).unwrap_or_else(|e| {
        error!("initialising config: {}", e);
        process::exit(1);
    });
    let config = Arc::new(Conf::load(&conf_path).unwrap_or_else(|e| {
        error!("loading config: {}", e);
        process::exit(1);
    }));

    println!("{:#?}", config);

    http::listen(config);
}

fn expand_tilde(path: &Path) -> PathBuf {
    match (BaseDirs::new(), path.strip_prefix("~")) {
        (Some(bd), Ok(stripped)) => bd.home_dir().join(stripped),
        _ => path.to_owned(),
    }
}

fn init_conf(conf_path: &PathBuf) -> Result<(), Error>{
    create_dir_if_missing(conf_path.parent().unwrap())?;
    if !conf_path.exists() {
        info!("creating default config '{}'", conf_path.display());
        Conf::default().write(&conf_path)?;
    }
    Ok(())
}

fn create_dir_if_missing(dir: &Path) -> Result<(), Error> {
    let dir_str = dir.to_str().unwrap();
    let exists = dir_str.is_empty() || dir.exists();
    if !exists {
        info!("creating directory '{}'", dir_str);
        fs::create_dir_all(dir)?;
    }
    Ok(())
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
