use rust_web_server::ThreadPool;
use std::{
    io::{prelude::*, BufReader},
    net::{TcpListener, TcpStream}, fs,
    thread::{self, JoinHandle},
    time::Duration, sync::Arc,
};

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let buf_reader = BufReader::new(&mut stream);

    // Retrieve the header of request
    let header = buf_reader.lines().next().unwrap().unwrap();

    let (status_line, filename) = match &header[..] {
        "GET / HTTP/1.1" =>  {
            println!("Route: /");
            ("HTTP/1.1 200 OK", "index.html")
        },
        "GET /sleep HTTP/1.1" => {
            println!("Route: /sleep");
            thread::sleep(Duration::from_secs(10));
            ("HTTP/1.1 200 OK", "index.html")
        },
        _ => {
            println!("Route /error");
            ("HTTP/1.1 404 NOT FOUND", "error.html")
        },
    };

    let contents = fs::read_to_string(filename).unwrap();
    let length = contents.len();

    let response =
        format!("{status_line}\r\nContent-Length: {length}\r\n\r\n{contents}");

    stream.write_all(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
