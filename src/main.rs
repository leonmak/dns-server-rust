use std::net::UdpSocket;

use dns_starter_rust::runner;

#[allow(dead_code)]
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    runner(udp_socket)
}
