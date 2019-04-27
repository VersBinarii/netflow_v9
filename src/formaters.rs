use byteorder::{BigEndian, ReadBytesExt};

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

pub fn fmt_ipv4(b: &[u8]) -> String {
    format!(r#""{}.{}.{}.{}""#, b[0], b[1], b[2], b[3])
}

pub fn fmt_ipv6(b: &[u8]) -> String {
    format!(
        r#""{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}:{:02x}{:02x}""#,
        b[0],
        b[1],
        b[2],
        b[3],
        b[4],
        b[5],
        b[6],
        b[7],
        b[8],
        b[9],
        b[10],
        b[11],
        b[12],
        b[13],
        b[14],
        b[15]
    )
}

pub fn fmt_int(mut b: &[u8]) -> String {
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

    format!("\"{}\"", val)
}

pub fn fmt_tcp_flags(b: &[u8]) -> String {
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

    format!("\"{}\"", res)
}
