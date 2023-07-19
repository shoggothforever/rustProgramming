use std::net::TcpListener;
use std::sync::Arc;
use std::thread;
use socket::{handle_stream, ThreadPool};

fn main() {
    let listener=Arc::new(TcpListener::bind("127.0.0.1:7890").unwrap());
    let pool=ThreadPool::new(4);
    // for stream in listener.incoming().take(2) {
    for stream in listener.incoming(){
        let s = stream.unwrap();
        pool.execute(|| {
            println!("established the connect");
            handle_stream(s);
        });
    };
}
