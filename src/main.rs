mod html_response_files;
mod response_files;

use clap::Parser;
use html_response_files::HtmlResponseFiles;
use log::info;
use mime_guess;
use regex::Regex;
use response_files::ResponseFiles;
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    net::{TcpListener, TcpStream},
};
use walkdir::{DirEntry, WalkDir};

#[derive(Parser)]
struct CliArgs {
    // path: Option<std::path::PathBuf>,
    #[arg(short, long)]
    port: Option<i16>,
}

fn main() {
    env_logger::init();
    let args = CliArgs::parse();
    start_server(args);
}

fn start_server(args: CliArgs) {
    let port = args.port.unwrap_or(7878).to_string();

    let listener = TcpListener::bind(format!("127.0.0.1:{port}")).unwrap();

    info!("Start server on 127.0.0.1:{}", port);

    for stream in listener.incoming() {
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

    if let Some(x) = get_request_regexp.captures(&http_request[0]) {
        let req_url = rem_first(x.get(1).unwrap().as_str());

        let paths = all_paths_walkdir();

        if paths.contains(&String::from(req_url)) {
            handle_exist_file(req_url, stream);
        } else {
            let html_response_files = HtmlResponseFiles {};
            response_file(
                stream,
                "HTTP/1.1 404 Not Found".to_string(),
                html_response_files.not_found(),
            )
        }
    } else {
        let html_response_files = HtmlResponseFiles {};
        response_file(
            stream,
            "HTTP/1.1 400 Bad Request".to_string(),
            html_response_files.not_get_request_method(),
        )
    }
}

fn is_hidden(entry: &DirEntry) -> bool {
    let file_name = entry.file_name().to_str().unwrap();

    file_name.starts_with(".") || entry.file_type().is_dir()
}

fn all_paths_walkdir() -> Vec<String> {
    let walker = WalkDir::new("./").into_iter();

    let mut paths: Vec<String> = Vec::new();

    for entry in walker.filter_map(|e| e.ok()) {
        if !is_hidden(&entry) {
            paths.push(String::from(
                entry.path().to_str().unwrap().replace("./", ""),
            ))
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
    let status_line = "HTTP/1.1 200 OK";
    let guess = mime_guess::from_path(&file_path);
    if let Some(mime) = guess.first() {
        let body_content = fs::read_to_string(format!("./{file_path}")).unwrap();
        let length = body_content.len();
        let mime_text = mime.to_string();
        let response = format!(
            "{status_line}\r\nContent-Length: {length}\r\nContent-Type: {mime_text}\r\n\r\n{body_content}"
        );
        stream.write_all(response.as_bytes()).unwrap();
    } else {
        let html_response_files = HtmlResponseFiles {};
        response_file(
            stream,
            "HTTP/1.1 400 Bad Request".to_string(),
            html_response_files.not_guessed_mime_type(),
        )
    }
}

fn response_file(mut stream: TcpStream, status_line: String, file: &[u8]) {
    let str_body_content = String::from_utf8_lossy(file);
    let length = file.len();
    let response = format!("{status_line}\r\nContent-Length: {length}\r\nContent-Type: text/html\r\n\r\n{str_body_content}");
    stream.write_all(response.as_bytes()).unwrap();
}
