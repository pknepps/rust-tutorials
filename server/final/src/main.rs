use std::{
    fs,
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream},
    thread,
    time::Duration,
};

use regex::Regex;

use r#final::ThreadPool;

static NOT_FOUND: (&str, &str) = ("HTTP/1.1 404 NOT FOUND", "404.html");

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    // Limits the number of threads that an spawn.
    let pool = ThreadPool::new(3);
    
    for stream in listener.incoming() {
        let stream = stream.unwrap();
        
        // Puts a new thread into the pool and executes it once there are
        // threads free.
        pool.execute(|| {
            handle_connection(stream);
        });
    }

    println!("Shutting down");
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);
    let request_line = buf_reader.lines().next().unwrap().unwrap();
    
    let (mut status_line, filename) = match &request_line[..] {
        "GET / HTTP/1.1" => ("HTTP/1.1 200 OK", "index.html"),
        "GET /sleep HTTP/1.1" => {
            thread::sleep(Duration::from_secs(5));
            ("HTTP/1.1 200 OK", "index.html")
        },
        request => parse_request(request),
    };

    println!("reading {filename}");
    let contents = fs::read(filename);
    let contents = match contents {
        Ok(contents) => contents,
        Err(_) => {
            status_line = NOT_FOUND.0;
            fs::read(NOT_FOUND.1).unwrap()
        },
    };
    let length = contents.len();

    let response = format!("{status_line}\r\nContent-Length: {length}\r\n\r\n");
    
    stream.write(response.as_bytes()).unwrap();
    stream.write(&contents).unwrap();
}

fn parse_request(request: &str) -> (&str, &str) {
    if !request.starts_with("GET") {
        return NOT_FOUND;
    }

    let regex = Regex::new(r"/(\S*)").unwrap();
    
    let file = regex.find(request).unwrap().as_str().trim();
    ("HTTP/1.1 200 OK", &file[1..])
}
