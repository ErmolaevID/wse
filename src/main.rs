use std::{net::{TcpListener, TcpStream}, io::{BufReader, BufRead}, fs};

use clap::Parser;
use regex::Regex;
use walkdir::{WalkDir, DirEntry};

#[derive(Parser)]
struct CliArgs {
    path: std::path::PathBuf,

}

enum Message {
	Move(i32),
	Write(String),
}

fn main() {
    // let args = CliArgs::parse();
    // start_server();

    let mut v = Message::Move(2);

	match v {
		Message::Move(msg) => println!("{}", msg),
		Message::Write(_) => println!("OK"),
	}
}

fn start_server() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let http_request: Vec<_> = buf_reader
        .lines()
        .map(|result| result.unwrap())
        .take_while(|line| !line.is_empty())
        .collect();

    let re = Regex::new(r"^GET (.*)[ ].*").unwrap();

    let req_url = re.captures(&http_request[0]).unwrap().get(1).unwrap().as_str();
    
    let req_url_without_first = rem_first(req_url);

    println!("{:#?}", all_paths_walkdir());
    let paths = all_paths_walkdir();

    println!("Request: {:#?}", req_url);
    if paths.contains(&String::from(req_url_without_first)) {
        println!("Found file");
    } else {
        println!("Not found file");
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
