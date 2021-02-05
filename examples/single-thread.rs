//! A simple single-threaded design
//!
//! Run this example server and
//! test the connection with `curl localhost:4321`
//!
//! Learnings:
//!
//! 1. Nonblocking stdin, for purposes of this example, would require another thread
//! While this isn't particularly an issue in this example, it would add extraneous
//! code--and I may as well use a TcpListener so the example code looks more like
//! the end goal for clarity.

use std::net::TcpListener;
use std::thread;
use std::time::Duration;

fn main() {
    const DELAY: Duration = Duration::from_secs(1);

    let listener = TcpListener::bind("127.0.0.1:4321").unwrap();
    listener.set_nonblocking(true).unwrap();

    loop {
        match listener.accept() {
            Ok((_socket, addr)) => println!("new client: {:?}", addr),
            Err(e) => println!("couldn't get client: {:?}", e),
        }
        thread::sleep(DELAY);
    }
}