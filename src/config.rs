use crate::{Error, Result};
use std::{fs, str::FromStr, sync::OnceLock};

pub fn config() -> &'static Config {
    static INSTANCE: OnceLock<Config> = OnceLock::new();

    INSTANCE.get_or_init(|| {
        Config::load_from_env()
            .unwrap_or_else(|ex| panic!("Fatal while loading conf. Reason: {ex:?}"))
    })
}

#[allow(non_snake_case)]
pub struct Config {
    pub PWD_KEY: Vec<u8>,
    pub TOKEN_KEY: Vec<u8>,
    pub TOKEN_DURATION_SEC: f64,
    pub DB_URL: String,
    pub WEB_FOLDER: String,
}

impl Config {
    fn load_from_env() -> Result<Config> {
        Ok(Config {
            PWD_KEY: get_env_b64u_as_u8s("SERVICE_PWD_KEY")?,
            TOKEN_KEY: get_env_b64u_as_u8s("SERVICE_TOKEN_KEY")?,
            TOKEN_DURATION_SEC: get_env_parse::<f64>("SERVICE_TOKEN_DURATION_SEC")?,
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

fn get_env_b64u_as_u8s(name: &'static str) -> Result<Vec<u8>> {
    base64_url::decode(&get_env(name)?).map_err(|_| Error::ConfigWrongFormat(name))
}

fn get_env_parse<T: FromStr>(name: &'static str) -> Result<T> {
    get_env(name)?
        .parse::<T>()
        .map_err(|_| Error::ConfigWrongFormat(name))
}
