use byteorder::{BigEndian, ByteOrder};
use std::net::UdpSocket;

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

    fn set_is_resp(&mut self, qr: bool) {
        self.qr = qr;
    }

    fn set_id(&mut self, id: u16) {
        self.id = id;
    }
    fn set_num_questions(&mut self, qdcount: u16) {
        self.qdcount = qdcount;
    }

    fn write_bytes(&self, bytes: &mut [u8; 512]) {
        BigEndian::write_u16(&mut bytes[0..2], self.id);
        let flags = (self.qr as u16) << 15
            | (self.opcode as u16) << 11
            | (self.aa as u16) << 10
            | (self.tc as u16) << 9
            | (self.rd as u16) << 8
            | (self.ra as u16) << 7
            | (self.z as u16) << 4
            | (self.rcode as u16);
        BigEndian::write_u16(&mut bytes[2..4], flags);
        BigEndian::write_u16(&mut bytes[4..6], self.qdcount);
        BigEndian::write_u16(&mut bytes[6..8], self.ancount);
        BigEndian::write_u16(&mut bytes[8..10], self.nscount);
        BigEndian::write_u16(&mut bytes[10..12], self.arcount);
    }
}

#[derive(Debug)]
struct DnsQuestion {
    qname: String,
    qtype: u16,
    qclass: u16,
}

impl DnsQuestion {
    fn from_bytes(buffer: &[u8], offset: &mut usize) -> Self {
        let mut qname = String::new();
        let i = offset;
        loop {
            // 1st byte is length of label
            let len = buffer[*i] as usize;
            *i += 1;
            if len == 0 {
                break;
            }
            qname.push_str(&String::from_utf8_lossy(&buffer[*i..*i + len]));
            qname.push('.');
            *i += len;
        }
        qname.pop(); // Remove the trailing dot

        let qtype = BigEndian::read_u16(&buffer[*i..*i + 2]);
        *i += 2;
        let qclass = BigEndian::read_u16(&buffer[*i..*i + 2]);

        DnsQuestion {
            qname,
            qtype,
            qclass,
        }
    }

    fn write_bytes(&self, buf: &mut Vec<u8>) {
        // Write the domain name
        for label in self.qname.split(|char| char == '.') {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0); // Null terminator for the domain name

        // Write the query type
        buf.extend_from_slice(&self.qtype.to_be_bytes());

        // Write the query class
        buf.extend_from_slice(&self.qclass.to_be_bytes());
    }
}

fn main() {
    // You can use print statements as follows for debugging, they'll be visible when running tests.
    println!("Logs from your program will appear here!");

    // Uncomment this block to pass the first stage
    let udp_socket = UdpSocket::bind("127.0.0.1:2053").expect("Failed to bind to address");
    let mut buf = [0; 512];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                let header_size = 12;
                let mut offset = header_size; // header size, use for iterating questions
                let mut header = DnsHeader::from_bytes(&buf[..offset]);

                let mut questions = Vec::new();
                for _ in 0..header.qdcount {
                    let question = DnsQuestion::from_bytes(&buf, &mut offset);
                    println!("question read: {:?}", question);
                    questions.push(question);
                }

                // Set the response flag in the header
                header.set_is_resp(true);
                header.set_id(header.id);
                header.set_num_questions(questions.len() as u16);
                println!("num questions read: {:?}", questions.len());

                // write header to first 12 bytes
                header.write_bytes(&mut buf);

                // write questions to buffer
                let mut resp: Vec<u8> = Vec::new();
                for question in questions {
                    question.write_bytes(&mut resp);
                }

                assert!(resp.len() <= buf.len() - header_size);
                buf[header_size..header_size + resp.len()].copy_from_slice(&resp);

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
