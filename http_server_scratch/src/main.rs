use std::{io::{BufWriter, Read}, net::{ TcpListener,TcpStream }};
use std::io::prelude::*;



fn main() {
    
    // Bind to Address.
    let listener = TcpListener::bind("127.0.0.1:7878").expect("Failed to Bind the Port");

    println!("Server Runnin on PORT : {:?}",listener);

    for stream in listener.incoming() {
        let stream = stream.expect("failed to accept Connection!");
        handleConnection(stream);

    }

}


fn handleConnection(mut stream : TcpStream){
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