// Simulating a Slow Request in the Current Server Implementation

use std::fs;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use std::thread;
use std::time::Duration;

fn simulating_a_slow_request_in_the_current_server_implementation() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        handle_connection(stream);
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream.read(&mut buffer).unwrap();

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n";

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "hello.html")
    } else if buffer.starts_with(sleep) {
        thread::sleep(Duration::from_secs(5));
        ("HTTP/1.1 200 OK", "hello.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND", "404.html")
    };

    let contents = fs::read_to_string(filename).unwrap();

    let response = format!(
        "{}\r\nContent-Length: {}\r\n\r\n{}",
        status_line,
        contents.len(),
        contents
    );

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}

// Code Structure If We Could Spawn a Thread for Each Request

fn code_structure_if_we_could_spawn_a_thread_for_each_request() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        thread::spawn(|| {
            handle_connection(stream);
        });
    }
}

// Creating a Similar Interface for a Finite Number of Threads

use turning_our_single_threaded_server_into_a_multithreaded_server::ThreadPool7;

fn creating_a_similar_interface_for_a_finite_number_of_threads() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool7::new_7(4);

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute_7(|| {
            handle_connection(stream);
        });
    }
}

fn main() {
    creating_a_similar_interface_for_a_finite_number_of_threads();
}
