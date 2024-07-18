use byteorder::{BigEndian, ByteOrder};

#[allow(dead_code)]
pub struct DnsHeader {
    pub id: u16,      // Packet Identifier (ID)
    pub qr: bool,     // Query/Response Indicator (QR)
    pub opcode: u16,  // Operation Code (OPCODE)
    pub aa: bool,     // Authoritative Answer (AA)
    pub tc: bool,     // Truncation (TC)
    pub rd: bool,     // Recursion Desired (RD)
    pub ra: bool,     // Recursion Available (RA)
    pub z: u8,        // Reserved (Z)
    pub rcode: u8,    // Response Code (RCODE)
    pub qdcount: u16, // Question Count (QDCOUNT)
    pub ancount: u16, // Answer Record Count (ANCOUNT)
    pub nscount: u16, // Authority Record Count (NSCOUNT)
    pub arcount: u16, // Additional Record Count (ARCOUNT)
}

#[allow(dead_code)]
impl DnsHeader {
    pub fn from_bytes(buffer: &[u8]) -> Self {
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

    pub fn set_is_resp(&mut self, qr: bool) {
        self.qr = qr;
    }

    pub fn set_id(&mut self, id: u16) {
        self.id = id;
    }
    pub fn set_num_questions(&mut self, qdcount: u16) {
        self.qdcount = qdcount;
    }
    pub fn set_answer_count(&mut self, ancount: u16) {
        self.ancount = ancount;
    }

    pub fn write_bytes(&self, bytes: &mut [u8; 1024]) {
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
    // new
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
