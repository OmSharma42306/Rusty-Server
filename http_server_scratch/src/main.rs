use std::{io::{ Read }, net::{ TcpListener,TcpStream }};
use std::io::prelude::*;
use std::thread;
mod thread_pool;
use thread_pool::ThreadPool;


fn main() {
    
    // Bind to Address.
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to Bind the Port");

    println!("Server Runnin on PORT : {:?}",listener);

    let pool = ThreadPool::new(4); // here i am createing

    for stream in listener.incoming() {
        let stream = stream.expect("failed to accept Connection!");
        
        // creating thread for per connection , so they process multiple request on their own time, no need to wait for one request to complete after other.
        // thread::spawn(||{
        //     handle_Connection(stream);
        // });

        pool.execute(|| handle_Connection(stream));
        
    }

}


fn handle_Connection(mut stream : TcpStream){
    let mut buffer = [0;1024]; // buffer for request bytes.

    // Read from Client
    stream.read(&mut buffer).expect("Failed to Read Stream");

    // printing raw requests.
    println!("Request : {}",String::from_utf8_lossy(&buffer[..]));

    // creating a simple raw response.
    let response = "HTTP/1.1 200 OK\r\n\r\n<html><h1>Hello From Rust!</h1></html>";

    // send this response to client
    // stream.write(response.as_bytes()).expect("Failed to Write Response");
    stream.write(response.as_bytes()).expect("Failed to write response");
    stream.flush().expect("Failed to Flush Stream!");



}