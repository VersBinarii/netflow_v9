use byteorder::{BigEndian, ReadBytesExt};

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
