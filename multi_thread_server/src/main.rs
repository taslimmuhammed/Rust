use std::time::Duration;
use std::{fs, thread};
use std::io::prelude::*;
use std::net::{TcpListener, TcpStream};
use multi_thread_server::ThreadPool;
fn main() {
    println!("Server running......");
    let listener: TcpListener = TcpListener::bind("127.0.0.1:7878").unwrap();
    
    let pool  = ThreadPool::new(4);
 
    //take(n) creates an iterator that yields the first n elements
    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();
        //instead of calling thread::spawn 
        // threadd:: spawn will create multiple thread with is not good
        pool.execute(||{
            handle_connection(stream)
        });
        
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024]; //arrays of 0s 1024 long

    stream.read(&mut buffer).unwrap();
    //println!("{}",String::from_utf8_lossy(&buffer[..]));

    let get = b"GET / HTTP/1.1\r\n";
    let sleep = b"GET /sleep HTTP/1.1\r\n"; //getting sleep path

    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK", "index.html")
    }else if buffer.starts_with(sleep){
       thread::sleep(Duration::from_secs(5));
       ("HTTP/1.1 200 OK", "index.html")
    }else {
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