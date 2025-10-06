use std::net::TcpListener;

fn main() {
    let listener: TcpListener = TcpListener::bind("127.0.0.1:6379").unwrap();
    println!("listner {:?}", listener);

    for stream in listener.incoming() {
        match stream {
            Ok(_stream) => {
                println!("Accepted a new connection {:?}", _stream);
            }
            Err(e) => {
                print!("Error Occured {}", e);
            }
        }
    }
}
