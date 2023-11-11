use once_cell::sync::Lazy;
use openapi::apis::configuration::Configuration;

#[cfg(debug_assertions)]
pub(crate) static AUTH_FILE: &str = "./data/config/auth.json";
#[cfg(not(debug_assertions))]
pub(crate) static AUTH_FILE: &str = "/config/auth.json";

#[cfg(debug_assertions)]
pub(crate) static DOWNLOAD_PATH: &str = "./data/downloads";
#[cfg(not(debug_assertions))]
pub(crate) static DOWNLOAD_PATH: &str = "/downloads";

pub(crate) static API_CONFIG: Lazy<Configuration> = Lazy::new(Configuration::new);
