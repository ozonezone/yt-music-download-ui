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

pub(crate) static DENO_SERVER_PORT: &str = "3031";

pub(crate) static API_CONFIG: Lazy<Configuration> = Lazy::new(|| Configuration {
    base_path: format!("http://localhost:{}", DENO_SERVER_PORT),
    ..Default::default()
});
