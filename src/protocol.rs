use std::io;
use tokio::io::{AsyncReadExt as _, AsyncWriteExt as _};
use tokio::net::TcpStream;

pub async fn read_varint(stream: &mut TcpStream) -> io::Result<i32> {
    let mut result = 0;
    let mut position = 0;

    loop {
        let byte = stream.read_u8().await?;
        result |= ((byte & 0b0111_1111) as i32) << position;
        position += 7;

        if byte & 0b1000_0000 == 0 {
            break;
        }

        if position >= 32 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                "VarInt is too big",
            ));
        }
    }

    Ok(result)
}

pub async fn write_varint_to_vec(vec: &mut Vec<u8>, mut value: i32) {
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

pub async fn write_response(stream: &mut TcpStream, response: &str) -> io::Result<()> {
    let mut response_array = Vec::new();
    write_varint_to_vec(&mut response_array, 0x00).await;
    write_varint_to_vec(&mut response_array, response.len() as i32).await;
    response_array.extend_from_slice(response.as_bytes());

    let mut length = Vec::new();
    write_varint_to_vec(&mut length, response_array.len() as i32).await;

    stream.write_all(&length).await?;
    stream.write_all(&response_array).await?;
    stream.flush().await?;
    Ok(())
}

pub async fn send_pong(stream: &mut TcpStream, ping_data: &[u8], n: usize) -> io::Result<()> {
    let mut response_array = Vec::new();
    write_varint_to_vec(&mut response_array, 9).await;
    write_varint_to_vec(&mut response_array, 0x01).await;
    response_array.extend_from_slice(&ping_data[n - 8..n]);
    stream.write_all(&response_array).await?;
    stream.flush().await?;
    Ok(())
}
