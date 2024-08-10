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
