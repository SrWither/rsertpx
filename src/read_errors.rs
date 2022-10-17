use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::exit;
use toml;

use crate::read_cfg::read_cfg;

#[derive(Deserialize, Clone)]
pub struct ErrorPage {
    pub error: u16,
    pub file: String,
}

fn read_errors() -> Vec<ErrorPage> {
    let routes_file = "./data/error_page.toml";

    let routes_content = match fs::read_to_string(routes_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Archivo no encontrado `{}`", routes_file);
            exit(1);
        }
    };

    let errors_table: HashMap<String, Vec<ErrorPage>> = toml::from_str(&routes_content).unwrap();
    let errors: &[ErrorPage] = &errors_table["error_page"];

    return errors.to_vec();
}

pub fn error_404() -> String {
    let errors = read_errors();
    let config_data = read_cfg();

    for error in errors {
        if error.error == 404 {
            return format!("{}/{}",config_data.config.base_dir,error.file)
        } else {
            println!("archivo 404 no encontrado")
        }
    }
    return "error".to_string()
} 