mod dns;

use dns::*;
use std::net::UdpSocket;

pub fn runner(udp_socket: UdpSocket) {
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
