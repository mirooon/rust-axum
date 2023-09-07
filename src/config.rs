use crate::{Error, Result};
use std::{env, fs, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("Fatal while loading conf. Reason: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub DB_URL: String,
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            DB_URL: get_env("SERVICE_DB_URL")?,
            WEB_FOLDER: get_env("SERVICE_WEB_FOLDER")?,
        })
    }
}

// fn get_env(name: &'static str) -> Result<String> {
//     env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
// }

fn get_env(name: &'static str) -> Result<String> {
    let content = fs::read_to_string("cargo/.config.toml").unwrap();
    let config: toml::Value = toml::from_str(&content).unwrap();

    if let Some(env_val) = config.get("env").and_then(|env| env.get(name)) {
        // println!("ENV: name: {}, val: {}", name, env_val.as_str().unwrap());
        return Ok(env_val.to_string().replace("\"", ""));
        // env_val.to_string().ok_or(Error::ConfigMissingEnv(name))
    }
    return Err(Error::ConfigMissingEnv(name));

    // DOES NOT WORK
    // env::var(name).map_err(|_| Error::ConfigMissingEnv(name))
}
