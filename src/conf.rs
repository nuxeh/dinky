use failure::Error;
use std::fs;
use std::path::{Path, PathBuf};
use toml;
use crate::db::DbType;

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Settings {
    pub bind: String,
    pub port: usize,
}

#[derive(Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Hash {
    pub length: usize,
    pub salt: String,
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Database {
    pub kind: DbType,
    pub path: String,
}

impl Default for Settings {
    fn default() -> Self {
        Self {
            port: 4444,
            bind: "127.0.0.1".to_string(),
        }
    }
}

impl Default for Hash {
    fn default() -> Self {
        Self {
            length: 10,
            salt: "dinkysalt123".to_string(),
        }
    }
}

#[derive(Default, Clone, Serialize, Deserialize)]
#[serde(default)]
pub struct Conf {
    pub settings: Settings,
    pub hash: Hash,
    pub database: Database,
}

impl Conf {
    pub fn load(path: impl AsRef<Path>) -> Result<Self, Error> {
        let conf = fs::read_to_string(path.as_ref())?;
        let conf: Conf = toml::de::from_str(&conf)?;
        Ok(conf)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    /// test that the example configuration file parses without error
    fn example_conf_load() {
        let _ = Conf::load(&PathBuf::from("example.config.toml")).unwrap();
    }

    #[test]
    /// test that the example configuration matches default values
    fn example_conf_matches_generated_default() {
        let example = fs::read_to_string("example.config.toml").unwrap();
        let default = toml::ser::to_string(&Conf::default()).unwrap();

        // print diff (on failure)
        println!("Configuration diff (- example, + default):");
        for diff in diff::lines(&example, &default) {
            match diff {
                diff::Result::Left(l) => println!("-{}", l),
                diff::Result::Both(l, _) => println!(" {}", l),
                diff::Result::Right(r) => println!("+{}", r)
            }
        }

        assert_eq!(default, example);
    }
}
