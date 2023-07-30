use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead, Write}, fs};
use clap::Parser;
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

#[derive(Parser)]
struct CliArgs {
    path: Option<std::path::PathBuf>,
    port: Option<i16>,
}

fn main() {
    let args = CliArgs::parse();
    start_server(args);
}

fn start_server(args: CliArgs) {
    let port = if args.port.is_some() {
        args.port.unwrap().to_string()
    } else {
        String::from("7878")
    };

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let get_request_regexp = Regex::new(r"^GET (.*)[ ].*").unwrap();

    let req_url = rem_first(get_request_regexp.captures(&http_request[0]).unwrap().get(1).unwrap().as_str());
    
    let paths = all_paths_walkdir();

    if paths.contains(&String::from(req_url)) {

        let status_line = "HTTP/1.1 200 OK";
        let body_content = fs::read_to_string(format!("./{req_url}")).unwrap();
        let length = body_content.len();   
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: application/toml\r\n\r\n{body_content}");
        stream.write_all(response.as_bytes()).unwrap();

    } else {
        let status_line = "HTTP/1.1 404 Not Found";
        let body_content = fs::read_to_string("./pages/404.html").unwrap();
        let length = body_content.len();   
        let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{body_content}");
        stream.write_all(response.as_bytes()).unwrap();
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    entry.file_name()
        .to_str()
        .map(|s| s.starts_with("target") || s.starts_with(".git"))
        .unwrap_or(false)
}

fn all_paths_walkdir() -> Vec<String> {
    let walker = WalkDir::new("./").into_iter();

    let mut paths: Vec<String> = Vec::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {
        paths.push(String::from(entry.unwrap().path().to_str().unwrap()).replace("./", ""));
    };

    return paths;
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}
