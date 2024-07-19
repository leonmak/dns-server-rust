use std::env;
use std::net::{IpAddr, UdpSocket};

use dns_starter_rust::runner;

#[allow(dead_code)]
fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");
    let args: Vec<String> = env::args().collect();
    println!("{args:?}");
    let addr_port_str: Vec<&str> = args.get(2).unwrap().split(":").collect();
    let resolver_ip = addr_port_str.get(0).unwrap().parse::<IpAddr>().unwrap();
    let resolver_port = addr_port_str.get(1).unwrap().parse::<u16>().unwrap();
    println!("{resolver_ip:?}");
    let socket_addr = Some(std::net::SocketAddr::new(resolver_ip, resolver_port));
    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    runner(udp_socket, socket_addr);
}
