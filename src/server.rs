use serde_json::json;
use std::io;
use tokio::io::AsyncReadExt as _;
use tokio::net::TcpStream;

use crate::models::{Description, Players, StatusResponse, Version, PROTOCOL_VERSION};
use crate::protocol::{read_varint, send_pong, write_response};

pub async fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    let _packet_length = read_varint(&mut stream).await?;
    let packet_id = read_varint(&mut stream).await?;

    if packet_id == 0x00 {
        let _protocol_version = read_varint(&mut stream).await?;
        let hostname_len = read_varint(&mut stream).await?;
        let mut hostname = vec![0; hostname_len as usize];
        stream.read_exact(&mut hostname).await?;
        let mut port_bytes = [0u8; 2];
        stream.read_exact(&mut port_bytes).await?;
        let _port = u16::from_be_bytes(port_bytes);
        let next_state = read_varint(&mut stream).await?;

        match next_state {
            1 => handle_status(&mut stream).await?,
            2 => handle_login(&mut stream).await?,
            _ => {}
        }
    }

    Ok(())
}

async fn handle_status(stream: &mut TcpStream) -> io::Result<()> {
    let mut request = vec![0; 1024];
    let n = stream.read(&mut request).await?;
    request.truncate(n);

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

    write_response(stream, &json!(status).to_string()).await?;

    // Handle ping
    let mut ping_data = vec![0; 1024];
    if let Ok(n) = stream.read(&mut ping_data).await {
        ping_data.truncate(n);
        if n >= 9 {
            send_pong(stream, &ping_data, n).await?;
        }
    }

    Ok(())
}

async fn handle_login(stream: &mut TcpStream) -> io::Result<()> {
    let mut login_data = vec![0; 1024];
    let n = stream.read(&mut login_data).await?;
    login_data.truncate(n);

    let disconnect_msg = json!({
        "text": "§c§lThis is a fake server!\n§eIt only responds to ping requests."
    })
    .to_string();

    write_response(stream, &disconnect_msg).await?;

    Ok(())
}
