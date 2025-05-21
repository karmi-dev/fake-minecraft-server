use byteorder::{BigEndian, ReadBytesExt};
use serde::Serialize;
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

const PROTOCOL_VERSION: i32 = 762; // Minecraft 1.19.4

#[derive(Serialize)]
struct StatusResponse {
    version: Version,
    players: Players,
    description: Description,
}

#[derive(Serialize)]
struct Version {
    name: String,
    protocol: i32,
}

#[derive(Serialize)]
struct Players {
    max: i32,
    online: i32,
}

#[derive(Serialize)]
struct Description {
    text: String,
}

fn read_varint(stream: &mut TcpStream) -> std::io::Result<i32> {
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

fn write_varint_to_vec(vec: &mut Vec<u8>, mut value: i32) {
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

fn handle_client(mut stream: TcpStream) -> std::io::Result<()> {
    // Read packet length
    let _packet_length = read_varint(&mut stream)?;

    // Read packet ID
    let packet_id = read_varint(&mut stream)?;

    // Read protocol version and state
    if packet_id == 0x00 {
        let _protocol_version = read_varint(&mut stream)?;
        let _hostname_len = read_varint(&mut stream)?;
        let mut hostname = vec![0; _hostname_len as usize];
        stream.read_exact(&mut hostname)?;
        let _port = stream.read_u16::<BigEndian>()?;
        let next_state = read_varint(&mut stream)?;

        match next_state {
            1 => {
                // Status
                // Read request packet (0x00)
                let _packet_length = read_varint(&mut stream)?;
                let _request_packet_id = read_varint(&mut stream)?;

                // Create status response
                let status = StatusResponse {
                    version: Version {
                        name: "1.19.4".to_string(),
                        protocol: PROTOCOL_VERSION,
                    },
                    players: Players { max: 20, online: 0 },
                    description: Description {
                        text: "§cFake Minecraft Server".to_string(),
                    },
                };

                let response = json!(status).to_string();
                let response_bytes = response.as_bytes();

                // Write response packet
                let mut response_packet = Vec::new();
                write_varint_to_vec(&mut response_packet, 0x00); // Packet ID
                write_varint_to_vec(&mut response_packet, response_bytes.len() as i32);
                response_packet.extend_from_slice(response_bytes);

                let mut final_packet = Vec::new();
                write_varint_to_vec(&mut final_packet, response_packet.len() as i32);
                final_packet.extend(response_packet);
                stream.write_all(&final_packet)?;
                stream.flush()?;

                // Handle ping if it comes
                let _ping_packet_length = read_varint(&mut stream)?;
                let ping_packet_id = read_varint(&mut stream)?;

                if ping_packet_id == 0x01 {
                    let mut payload = [0u8; 8];
                    stream.read_exact(&mut payload)?;

                    // Send pong
                    let mut pong_packet = Vec::new();
                    write_varint_to_vec(&mut pong_packet, 9); // Packet length (1 + 8)
                    write_varint_to_vec(&mut pong_packet, 0x01); // Packet ID
                    pong_packet.extend_from_slice(&payload);
                    stream.write_all(&pong_packet)?;
                    stream.flush()?;
                }
            }
            _ => {}
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() -> std::io::Result<()> {
    let listener = TcpListener::bind("0.0.0.0:25565")?;
    println!("Server listening on port 25565");

    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                tokio::spawn(async move {
                    if let Err(e) = handle_client(stream) {
                        eprintln!("Error handling client: {}", e);
                    }
                });
            }
            Err(e) => {
                eprintln!("Error accepting connection: {}", e);
            }
        }
    }

    Ok(())
}
