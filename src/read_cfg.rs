use serde_derive::Deserialize;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize)]
pub struct ConfigData {
    pub config: Config
}

#[derive(Deserialize)]
pub struct Config {
    pub ip: String,
    pub port: u16,
    pub base_dir: String,
    pub php: bool,
    pub php_path: String,
}

pub fn read_cfg() -> ConfigData {
    let config_file = "./data/config.toml";

    let config_content = match fs::read_to_string(config_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Archivo no encontrado `{}`", config_file);
            exit(1);
        }
    };

    let data: ConfigData = match toml::from_str(&config_content) {
        Ok(d) => d,
        Err(_) => {
            eprintln!("Datos no encontrado desde `{}`", config_file);
            exit(1);
        }
    };

    return data;
}