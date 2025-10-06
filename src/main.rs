use std::{
    io::Write,
    net::{TcpListener, TcpStream},
};

fn handle_stream(mut stream: TcpStream) {
    let msg = "+PONG\r\n";
    stream.write_all(msg.as_bytes()).unwrap();
}

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("Listening on {:?}", listener.local_addr().unwrap());

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Accepted connection from {}", stream.peer_addr().unwrap());
                handle_stream(stream);
            }
            Err(e) => {
                eprintln!(" Error: {}", e);
            }
        }
    }
}
