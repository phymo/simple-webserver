use std::fs;
use std::thread;
use std::time::Duration;
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use simple_server::ThreadPool;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();
    let pool = ThreadPool::new(4);

    // The take method is defined in the Iterator trait and limits the iteration to the first four items at most.
    // The ThreadPool will go out of scope at the end of main, and the drop implementation will run.
    for stream in listener.incoming().take(4) {
        //For now, our handling of the stream consists of calling unwrap to terminate our program if the stream has any errors; if there aren’t any errors,
        let stream = stream.unwrap();


        pool.execute(|| {
            handle_connection(stream);
        });
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

    let content = fs::read_to_string(filename).unwrap();

    let response = format!("{}\r\nContent-Length: {}\r\n\r\n{}", status_line, content.len(), content);

    stream.write(response.as_bytes()).unwrap();
    // flush will wait and prevent the program from continuing until all the bytes are written to the connection
    stream.flush().unwrap();
}
