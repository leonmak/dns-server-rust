use byteorder::{BigEndian, ByteOrder};
use std::net::UdpSocket;

trait ToBytes {
    fn to_bytes(&self) -> Vec<u8>;
}

#[allow(dead_code)]
struct DnsHeader {
    id: u16,      // Packet Identifier (ID)
    qr: bool,     // Query/Response Indicator (QR)
    opcode: u16,  // Operation Code (OPCODE)
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
    fn from_bytes(buffer: &[u8]) -> Self {
        let id = BigEndian::read_u16(&buffer[0..2]);
        let flags = BigEndian::read_u16(&buffer[2..4]);

        let qr = (flags >> 15) & 0b1 != 0;
        let opcode = (flags >> 11) & 0b1111;
        let aa = (flags >> 10) & 0b1 != 0;
        let tc = (flags >> 9) & 0b1 != 0;
        let rd = (flags >> 8) & 0b1 != 0;
        let ra = (flags >> 7) & 0b1 != 0;
        let z = 0;
        let rcode = flags & 0b1111;

        let qdcount = BigEndian::read_u16(&buffer[4..6]);
        let ancount = BigEndian::read_u16(&buffer[6..8]);
        let nscount = BigEndian::read_u16(&buffer[8..10]);
        let arcount = BigEndian::read_u16(&buffer[10..12]);

        DnsHeader {
            id,
            qr,
            opcode,
            aa,
            tc,
            rd,
            ra,
            z,
            rcode: rcode as u8,
            qdcount,
            ancount,
            nscount,
            arcount,
        }
    }

    fn set_qr(&mut self, qr: bool) {
        self.qr = qr;
    }
}

fn set_17th_bit(bytes: &mut [u8; 12], value: bool) {
    let byte_index = 2;
    let bit_offset = 0;

    if value {
        bytes[byte_index] |= 1 << (7 - bit_offset);
    } else {
        bytes[byte_index] &= !(1 << (7 - bit_offset));
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 12];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);
                println!("{:?}", buf);
                // let mut header = DnsHeader::from_bytes(&buf);
                // header.set_qr(true);
                set_17th_bit(&mut buf, true);
                println!("{:?}", buf);

                udp_socket
                    .send_to(&buf, source)
                    .expect("Failed to send response");
            }
            Err(e) => {
                eprintln!("Error receiving data: {}", e);
                break;
            }
        }
    }
}
