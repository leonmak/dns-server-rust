use std::env;
use std::net::{IpAddr, Ipv4Addr, UdpSocket};

use dns_starter_rust::runner;

#[allow(dead_code)]
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();
    let resolver_ip = args
        .get(2)
        .unwrap_or(&"8.8.8.8".to_owned())
        .parse::<IpAddr>()
        .unwrap();
    println!("{resolver_ip:?}");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    runner(udp_socket, resolver_ip);
}
