use clap::Parser;
use regex::Regex;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use walkdir::{DirEntry, WalkDir};
use mime_guess;

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

    println!("Start server on 127.0.0.1:{}", port);

    for stream in listener.incoming() {
        println!("New TCP connection");
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(stream: TcpStream) {
    let buf_reader = BufReader::new(&stream);
    let http_request: Vec<String> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let get_request_regexp = Regex::new(r"^GET (.*)[ ].*").unwrap();

    let req_url = rem_first(
        get_request_regexp
            .captures(&http_request[0])
            .unwrap()
            .get(1)
            .unwrap()
            .as_str(),
    );

    let paths = all_paths_walkdir();

    println!("{:?}", paths);

    if paths.contains(&String::from(req_url)) {
        handle_exist_file(req_url, stream);
    } else {
        handle_not_exist_file(stream);
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    let filename = entry.file_name().to_str().unwrap();

    println!("sdasd");

    filename.starts_with("target") || filename.starts_with(".git") || is_dir(entry)
}

fn is_dir(entry: &DirEntry) -> bool {
    println!("{}", entry.file_type().is_dir());
    entry.file_type().is_dir()
}

fn all_paths_walkdir() -> Vec<String> {
    let walker = WalkDir::new("./").into_iter();

    let mut paths: Vec<String> = Vec::new();

    for entry in walker.filter_entry(|e| !is_hidden(e)) {

        let path = String::from(entry.unwrap().path().to_str().unwrap()).replace("./", "");

        if path != "" {
            paths.push(path)
        }

    }

    paths
}

fn rem_first(value: &str) -> &str {
    let mut chars = value.chars();
    chars.next();
    chars.as_str()
}

fn handle_exist_file(file_path: &str, mut stream: TcpStream) {
    println!("{}", file_path);
    let status_line = "HTTP/1.1 200 OK";
    let guess = mime_guess::from_path(&file_path);
    let mime = guess.first().unwrap().to_string();
    let body_content = fs::read_to_string(format!("./{file_path}")).unwrap();
    let length = body_content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: {mime}\r\n\r\n{body_content}");
    println!("Return existing file");
    stream.write_all(response.as_bytes()).unwrap();
}

fn handle_not_exist_file(mut stream: TcpStream) {
    let status_line = "HTTP/1.1 404 Not Found";
    let body_content: &'static [u8] = include_bytes!("../pages/404.html");
    let str_body_content = String::from_utf8_lossy(body_content);
    let length = body_content.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{str_body_content}");
    println!("Return 404 code");
    stream.write_all(response.as_bytes()).unwrap();
}