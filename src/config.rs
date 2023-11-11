use once_cell::sync::Lazy;
use openapi::apis::configuration::Configuration;

pub(crate) struct Config {
    pub(crate) config_path: &'static str,
    pub(crate) download_path: &'static str,
    pub(crate) deno_server_port: &'static str,
    pub(crate) language: Option<&'static str>,
}

#[cfg(debug_assertions)]
pub(crate) static CONFIG: Config = Config {
    config_path: "./data/config",
    download_path: "./data/downloads",
    deno_server_port: "3031",
    language: Some("ja"),
};
#[cfg(not(debug_assertions))]
pub(crate) static CONFIG: Config = Config {
    config_path: "/config",
    download_path: "/downloads",
    deno_server_port: "3031",
    language: Some("ja"),
};

pub(crate) static API_CONFIG: Lazy<Configuration> = Lazy::new(|| Configuration {
    base_path: format!("http://localhost:{}", CONFIG.deno_server_port),
    ..Default::default()
});
