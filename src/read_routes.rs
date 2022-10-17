use serde_derive::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::process::exit;
use toml;

#[derive(Deserialize, Clone)]
pub struct Route {
    pub path: String,
    pub file: String,
}

pub fn read_routes() -> Vec<Route> {
    let routes_file = "./data/routes.toml";

    let routes_content = match fs::read_to_string(routes_file) {
        Ok(c) => c,
        Err(_) => {
            eprintln!("Archivo no encontrado `{}`", routes_file);
            exit(1);
        }
    };

    let routes_table: HashMap<String, Vec<Route>> = toml::from_str(&routes_content).unwrap();
    let routes: &[Route] = &routes_table["route"];

    return routes.to_vec();
}