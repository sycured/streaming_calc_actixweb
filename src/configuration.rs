use std::env::var;

use log::warn;

pub async fn get_from_env_or_default(env_name: &str, default: String) -> String {
    var(env_name).unwrap_or_else(|_| {
        let default = default;
        warn!(
            "Using default {name}: {value}",
            name = env_name,
            value = default
        );
        default
    })
}

pub async fn app_ip() -> String {
    get_from_env_or_default("APP_IP", "127.0.0.1".to_string()).await
}

pub async fn app_port() -> String {
    get_from_env_or_default("APP_PORT", "8080".to_string()).await
}
