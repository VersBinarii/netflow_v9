use byteorder::{BigEndian, ByteOrder, ReadBytesExt};
use serde::Serialize;
use std::net::{Ipv4Addr, Ipv6Addr};

static TCP_FLAGS: [(u8, &str); 8] = [
    (0x01, "FIN"),
    (0x02, "SYN"),
    (0x04, "RST"),
    (0x08, "PSH"),
    (0x10, "ACK"),
    (0x20, "URG"),
    (0x40, "ECE"),
    (0x08, "CWR"),
];

#[derive(Debug)]
pub enum FmtReturn {
    Number(u64),
    Text(String),
}

impl Serialize for FmtReturn {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::ser::Serializer,
    {
        match self {
            FmtReturn::Number(n) => serializer.serialize_u64(*n),
            FmtReturn::Text(t) => serializer.serialize_str(t),
        }
    }
}

impl std::fmt::Display for FmtReturn {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            FmtReturn::Number(n) => write!(f, "{}", n),
            FmtReturn::Text(t) => write!(f, "{}", t),
        }
    }
}

pub fn fmt_ipv4(b: &[u8]) -> FmtReturn {
    FmtReturn::Text(Ipv4Addr::new(b[0], b[1], b[2], b[3]).to_string())
}

pub fn fmt_ipv6(b: &[u8]) -> FmtReturn {
    FmtReturn::Text(Ipv6Addr::from(BigEndian::read_u128(&b)).to_string())
}

pub fn fmt_int(mut b: &[u8]) -> FmtReturn {
    let val = match b.len() {
        1 => b[0] as u64,
        2 => {
            if let Ok(num) = b.read_u16::<BigEndian>() {
                num as u64
            } else {
                0 as u64
            }
        }
        4 => {
            if let Ok(num) = b.read_u32::<BigEndian>() {
                num as u64
            } else {
                0 as u64
            }
        }
        8 => {
            if let Ok(num) = b.read_u64::<BigEndian>() {
                num
            } else {
                0 as u64
            }
        }
        _ => 0,
    };

    FmtReturn::Number(val)
}

pub fn fmt_tcp_flags(b: &[u8]) -> FmtReturn {
    let mut res = String::new();
    for e in TCP_FLAGS.iter() {
        if b[0] & e.0 == e.0 {
            res.push_str(e.1);
            res.push('-');
        }
    }
    if res.is_empty() {
        res.push_str("None");
    }

    if res.ends_with('-') {
        res.pop();
    }

    FmtReturn::Text(res)
}
