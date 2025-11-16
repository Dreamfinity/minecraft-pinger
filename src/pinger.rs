use crate::minecraft_packet::{MinecraftPacketReader, MinecraftPacketWriter};
use std::io::Read;

const DEFAULT_PORT: u16 = 25565;
const DEFAULT_HOST: &str = "127.0.0.1";

pub(crate) fn pinger() -> Result<(), std::io::Error> {
    let args: Vec<String> = std::env::args().collect();

    let mut server_addr = DEFAULT_HOST;
    let mut server_port = DEFAULT_PORT;

    if args.len() > 1 {
        let maybe_addr = &args[1];
        if let Some((maybe_addr, maybe_port)) = maybe_addr.split_once(":") {
            server_addr = maybe_addr;
            server_port = maybe_port.parse::<u16>().unwrap_or(DEFAULT_PORT);
        }
    }

    let mut stream = std::net::TcpStream::connect(format!("{}:{}", server_addr, server_port))?;

    Vec::<u8>::new()
        .write_varint(0x0)?
        .write_varint(0x0)?
        .write_str(server_addr)?
        .write_ushort(server_port)?
        .write_varint(0x01)?
        .send_to(&mut stream)?;

    Vec::<u8>::new().write_varint(0x00)?.send_to(&mut stream)?;

    stream.read_varint()?;
    stream.read_varint()?;

    let server_info_len = stream.read_varint()?;

    let mut server_info: Vec<u8> = vec![0u8; server_info_len as usize];
    stream.read_exact(&mut server_info)?;
    println!("{:?}", String::from_utf8_lossy(server_info.as_ref()));
    Ok(())
}
