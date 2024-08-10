use std::net::IpAddr;

use byteorder::{BigEndian, ByteOrder};

#[derive(Debug)]
pub struct DnsQuestion {
    pub qname: String,
    pub qtype: u16,
    pub qclass: u16,
}

// add question section

fn get_pointer_name(pointer_offset: usize, buffer: &[u8]) -> String {
    if pointer_offset >= buffer.len() {
        return String::new();
    }

    let mut name = String::new();
    let mut i = pointer_offset;
    loop {
        let len = buffer[i] as usize;
        i += 1;
        if len == 0 {
            break;
        } else {
            name.push_str(&String::from_utf8_lossy(&buffer[i..i + len]));
            name.push('.');
            i += len;
        }
    }
    println!("ptr name: {:?}", name);
    name
}

impl DnsQuestion {
    pub fn from_bytes(buffer: &[u8], offset: &mut usize) -> Self {
        let i = offset;
        let mut name = String::new();
        loop {
            // 1st byte is length of label
            let len = buffer[*i] as usize;
            println!("len: {:?}", len);
            *i += 1;
            if len == 0 {
                break;
            } else if (len & 0xC0) == 0xC0 {
                // 11 leading is a pointer
                println!("pointer");
                let pointer_offset = ((len & 0x3F) << 8) | buffer[*i] as usize;
                *i += 1;
                let pointer_name = get_pointer_name(pointer_offset, buffer);
                name.push_str(&pointer_name);
            } else {
                name.push_str(&String::from_utf8_lossy(&buffer[*i..*i + len]));
                name.push('.');
                *i += len;
            }
        }
        name.pop(); // Remove the trailing dot

        let qtype = BigEndian::read_u16(&buffer[*i..*i + 2]);
        *i += 2;
        let qclass = BigEndian::read_u16(&buffer[*i..*i + 2]);
        *i += 2;

        DnsQuestion {
            qname: name,
            qtype,
            qclass,
        }
    }

    pub fn write_bytes(&self, buf: &mut Vec<u8>) {
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

// add answer section
#[derive(Debug)]
pub struct DnsAnswer {
    pub name: String,
    pub qtype: u16,
    pub qclass: u16,
    pub ttl: u32,
    pub data_len: u16,
    pub data: Vec<u8>,
}

impl DnsAnswer {
    pub fn set_ip_addr(&mut self, ip_addr: IpAddr) {
        self.data = match ip_addr {
            IpAddr::V4(ipv4) => ipv4.octets().to_vec(),
            IpAddr::V6(ipv6) => ipv6.octets().to_vec(),
        };
        self.data_len = self.data.len() as u16;
    }

    pub fn new() -> Self {
        DnsAnswer {
            name: String::new(),
            qtype: 0,
            qclass: 0,
            ttl: 0,
            data_len: 0,
            data: Vec::new(),
        }
    }

    #[allow(dead_code)]
    pub fn from_bytes(buffer: &[u8], offset: &mut usize) -> Self {
        // Parse the name
        let mut name = String::new();
        let i = offset;
        loop {
            let len = buffer[*i] as usize;
            *i += 1;
            if len == 0 {
                break;
            }
            name.push_str(&String::from_utf8_lossy(&buffer[*i..*i + len]));
            name.push('.');
            *i += len;
        }
        name.pop(); // Remove the trailing dot

        let qtype = BigEndian::read_u16(&buffer[*i..*i + 2]);
        *i += 2;
        let qclass = BigEndian::read_u16(&buffer[*i..*i + 2]);
        *i += 2;
        let ttl = BigEndian::read_u32(&buffer[*i..*i + 4]);
        *i += 4;
        let data_len = BigEndian::read_u16(&buffer[*i..*i + 2]);
        *i += 2;
        let data = buffer[*i..*i + data_len as usize].to_vec();
        *i += data_len as usize;

        DnsAnswer {
            name,
            qtype,
            qclass,
            ttl,
            data_len,
            data,
        }
    }

    pub fn write_bytes(&self, buf: &mut Vec<u8>) {
        // Write the domain name
        for label in self.name.split(|char| char == '.') {
            buf.push(label.len() as u8);
            buf.extend_from_slice(label.as_bytes());
        }
        buf.push(0); // Null terminator for the domain name

        // Write the query type
        buf.extend_from_slice(&self.qtype.to_be_bytes());

        // Write the query class
        buf.extend_from_slice(&self.qclass.to_be_bytes());

        // Write the TTL
        buf.extend_from_slice(&self.ttl.to_be_bytes());

        // Write the data length
        buf.extend_from_slice(&self.data_len.to_be_bytes());

        // Write the data
        buf.extend_from_slice(&self.data);
    }
}
