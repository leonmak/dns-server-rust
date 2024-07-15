use std::net::UdpSocket;

#[allow(dead_code)]
struct DnsHeader {
    id: u16,      // Packet Identifier (ID)
    qr: bool,     // Query/Response Indicator (QR)
    opcode: u8,   // Operation Code (OPCODE)
    aa: bool,     // Authoritative Answer (AA)
    tc: bool,     // Truncation (TC)
    rd: bool,     // Recursion Desired (RD)
    ra: bool,     // Recursion Available (RA)
    z: u8,        // Reserved (Z)
    rcode: u8,    // Response Code (RCODE)
    qdcount: u16, // Question Count (QDCOUNT)
    ancount: u16, // Answer Record Count (ANCOUNT)
    nscount: u16, // Authority Record Count (NSCOUNT)
    arcount: u16, // Additional Record Count (ARCOUNT)
}

#[allow(dead_code)]
impl DnsHeader {
    fn new() -> Self {
        DnsHeader {
            id: 1234,
            qr: false,
            opcode: 0,
            aa: false,
            tc: false,
            rd: false,
            ra: false,
            z: 0,
            rcode: 0,
            qdcount: 0,
            ancount: 0,
            nscount: 0,
            arcount: 0,
        }
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 1024];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                let response = [];
                udp_socket
                    .send_to(&response, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
