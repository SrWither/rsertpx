use std::process::Command;
use crate::read_cfg::read_cfg;

pub fn php_parse(file: &str) -> String {
    let cfg = read_cfg();
    let output = Command::new(cfg.config.php_path)
                     .arg(file)
                     .output()
                     .expect("php command or php file not found");


    let phpcontent = String::from_utf8_lossy(&output.stdout);
    
    return phpcontent.to_string()
}