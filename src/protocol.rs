use byteorder::ReadBytesExt;
use std::net::TcpStream;

pub fn read_varint(stream: &mut TcpStream) -> std::io::Result<i32> {
    let mut result = 0;
    let mut position = 0;

    loop {
        let byte = stream.read_u8()?;
        result |= ((byte & 0b0111_1111) as i32) << position;
        position += 7;

        if byte & 0b1000_0000 == 0 {
            break;
        }

        if position >= 32 {
            return Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }
    }

    Ok(result)
}

pub fn write_varint_to_vec(vec: &mut Vec<u8>, mut value: i32) {
    loop {
        let mut temp = (value & 0b0111_1111) as u8;
        value >>= 7;
        if value != 0 {
            temp |= 0b1000_0000;
        }
        vec.push(temp);
        if value == 0 {
            break;
        }
    }
}
