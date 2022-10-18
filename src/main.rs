mod read_cfg;
mod read_errors;
mod read_routes;
mod phpparse;

use phpparse::php_parse;
use threadpool::ThreadPool;

use crate::{read_cfg::read_cfg, read_errors::error_404, read_routes::read_routes};

use std::{
    fs,
    io::prelude::*,
    net::{TcpListener, TcpStream},
};

fn main() {
    let config_data = read_cfg();
    println!(
        "Server running on: {}:{}",
        config_data.config.ip, config_data.config.port
    );

    let listener = TcpListener::bind(format!(
        "{}:{}",
        config_data.config.ip, config_data.config.port
    ))
    .unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("shutdowning...");
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    let mut response: String;
    let mut count = 1;

    let routes = read_routes();
    let config_data = read_cfg();

    stream.read(&mut buffer).unwrap();

    for route in routes.iter() {
        let contents: String;
        let bufstr = String::from(std::str::from_utf8(&buffer).unwrap());

        let path = format!("GET {} HTTP/1.1\r\n", route.path);

        let (status_line, filename) = ("HTTP/1.1 200 OK", format!("{}/{}", config_data.config.base_dir, route.file));
        if config_data.config.php{
            contents = php_parse(&filename);
        } else {
            contents = fs::read_to_string(filename).unwrap();
        }

        response = format!(
            "{}\r\nContent-Length: {}\r\n\r\n{}",
            status_line,
            contents.len(),
            contents
        );

        if bufstr.contains(&path) {
            stream.write_all(response.as_bytes()).unwrap();
        } else if count == routes.iter().len() {

            let content404 = fs::read_to_string(error_404()).unwrap();
            let error_line = "HTTP/1.1 404 NOT FOUND";

            response = format!(
                "{}\r\nContent-Length: {}\r\n\r\n{}",
                error_line,
                content404.len(),
                content404
            );

            stream.write_all(response.as_bytes()).unwrap();
        }
        count += 1;
    }
    stream.flush().unwrap();
}
