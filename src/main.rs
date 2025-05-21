mod models;
mod protocol;

use byteorder::{BigEndian, ReadBytesExt};
use serde_json::json;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};

use crate::models::{Description, Players, StatusResponse, Version, PROTOCOL_VERSION};
use crate::protocol::{read_varint, write_varint_to_vec};

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
