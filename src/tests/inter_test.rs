mod dns;

use crate::dns::redis_runner;
use std::{net::UdpSocket, process::Command, thread};

#[test]
fn when_dig_get_header() {
    // Start the runner in a separate thread
    let runner_thread = thread::spawn(|| {
        let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
        let resolver_addr = None;
        redis_runner::handle(udp_socket, resolver_addr);
    });

    let output = Command::new("dig")
        .arg("@127.0.0.1")
        .arg("-p")
        .arg("2053")
        .arg("+noedns")
        .arg("codecrafters.io")
        .output()
        .expect("Failed to execute dig command");

    println!("Status: {}", output.status);
    println!("Stdout: {}", String::from_utf8_lossy(&output.stdout));
    println!("Stderr: {}", String::from_utf8_lossy(&output.stderr));

    let stdout = String::from_utf8_lossy(&output.stdout);
    assert!(stdout.contains(";; global options: +cmd"));
    assert!(stdout.contains(";; Got answer:"));
    assert!(stdout.contains(";; ->>HEADER<<- opcode: QUERY, status: NOERROR, id: "));
    assert!(stdout.contains(";; flags: qr rd; QUERY: 1, ANSWER: 1, AUTHORITY: 0, ADDITIONAL: 0"));
    assert!(stdout.contains(";; QUESTION SECTION:"));
    assert!(stdout.contains(";codecrafters.io."));
    assert!(stdout.contains(";; ANSWER SECTION:"));
    assert!(stdout.contains("codecrafters.io."));
    assert!(stdout.contains("IN"));
    assert!(stdout.contains("A"));
    assert!(stdout.contains(";; MSG SIZE  rcvd: 1024"));

    runner_thread.thread().unpark();
}
