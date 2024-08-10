use std::net::{SocketAddr, UdpSocket};
use trust_dns_resolver::{
    config::{NameServerConfig, ResolverConfig, ResolverOpts},
    Resolver,
};

use crate::dns::{
    header::DnsHeader,
    question::{DnsAnswer, DnsQuestion},
};

pub fn handle(udp_socket: UdpSocket, resolver_addr: Option<SocketAddr>) {
    let mut buf = [0; 1024];

    loop {
        match udp_socket.recv_from(&mut buf) {
            Ok((size, source)) => {
                println!("Received {} bytes from {}", size, source);

                // Read to header, questions
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
                header.set_answer_count(questions.len() as u16);
                // 0 (no error) if OPCODE is 0 (standard query) else 4 (not implemented)
                if header.opcode == 0 {
                    header.rcode = 0;
                } else {
                    header.rcode = 4;
                }

                // custom header
                println!("num questions read: {:?}", questions.len());

                // write header to first 12 bytes
                header.write_bytes(&mut buf);

                // write questions to buffer
                let mut resp: Vec<u8> = Vec::new();
                for question in &questions {
                    question.write_bytes(&mut resp);
                }

                let mut start_idx = header_size;
                // assert!(resp.len() <= buf.len() - header_size);
                buf[start_idx..start_idx + resp.len()].copy_from_slice(&resp);
                start_idx += resp.len();

                // write answer
                for question in &questions {
                    let mut answer = DnsAnswer::new();
                    answer.name = question.qname.clone();
                    answer.qtype = 1;
                    answer.qclass = 1;
                    answer.ttl = 60;

                    // mimic the data from resolved server
                    if let Some(addr) = resolver_addr {
                        let mut config = ResolverConfig::new();
                        config.add_name_server(NameServerConfig::new(
                            addr,
                            trust_dns_resolver::config::Protocol::Udp,
                        ));
                        let resolver = Resolver::new(config, ResolverOpts::default()).unwrap();
                        let resp = resolver.lookup_ip(answer.name.clone()).unwrap();
                        if let Some(resp_ip) = resp.iter().next() {
                            answer.set_ip_addr(resp_ip);
                        }
                    } else {
                        answer.data = vec![8, 8, 8, 8];
                        answer.data_len = answer.data.len() as u16;
                    }
                    // answer.set_ip_addr(resolver_ip);
                    resp.clear();
                    answer.write_bytes(&mut resp);
                    buf[start_idx..start_idx + resp.len()].copy_from_slice(&resp);
                    start_idx += resp.len();
                }

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
