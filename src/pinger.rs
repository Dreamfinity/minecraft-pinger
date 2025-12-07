use crate::minecraft_packet::{MinecraftPacketReader, MinecraftPacketWriter};
use std::io::Read;

const DEFAULT_PORT: u16 = 25565;
const DEFAULT_HOST: &str = "127.0.0.1";

#[derive(Debug, Clone)]
struct PingerArgs<'a> {
    server_addr: &'a str,
    server_port: u16,
    verbose: bool,
    timeout: u64,
}

impl<'a> Default for PingerArgs<'a> {
    fn default() -> Self {
        Self {
            server_addr: DEFAULT_HOST,
            server_port: DEFAULT_PORT,
            verbose: false,
            timeout: 30,
        }
    }
}

fn parse_args(args: &'_ [String]) -> Result<PingerArgs<'_>, std::io::Error> {
    let mut args_iter = args.iter();
    args_iter.next();

    let mut pinger_args = PingerArgs::default();

    while let Some(one_arg) = args_iter.next() {
        if !(one_arg.starts_with("--") || one_arg.starts_with("-")) {
            let maybe_addr = one_arg;
            if let Some((maybe_addr, maybe_port)) = maybe_addr.split_once(":") {
                pinger_args.server_addr = maybe_addr;
                pinger_args.server_port = maybe_port.parse::<u16>().unwrap_or(DEFAULT_PORT);
            } else {
                pinger_args.server_addr = maybe_addr;
                pinger_args.server_port = DEFAULT_PORT;
            }
        } else {
            match one_arg.as_str() {
                "-h" | "--host" => {
                    if let Some(maybe_addr) = args_iter.next() {
                        pinger_args.server_addr = maybe_addr
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("{one_arg} arg requires a string argument but none provided"),
                        ));
                    }
                }
                "-p" | "--port" => {
                    if let Some(maybe_port) = args_iter.next() {
                        if let Ok(port) = maybe_port.parse::<u16>() {
                            pinger_args.server_port = port;
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!(
                                    "{one_arg} arg requires a number argument but '{maybe_port}' provided"
                                ),
                            ));
                        }
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("{one_arg} arg requires a number argument but none provided"),
                        ));
                    }
                }
                "-v" | "--verbose" => pinger_args.verbose = true,
                "-t" | "--timeout" => {
                    if let Some(maybe_timeout) = args_iter.next() {
                        if let Ok(timeout) = maybe_timeout.parse::<u64>() {
                            pinger_args.timeout = timeout;
                        } else {
                            return Err(std::io::Error::new(
                                std::io::ErrorKind::InvalidInput,
                                format!(
                                    "{one_arg} arg requires a number argument but '{maybe_timeout}' provided"
                                ),
                            ));
                        }
                    } else {
                        return Err(std::io::Error::new(
                            std::io::ErrorKind::InvalidInput,
                            format!("{one_arg} arg requires a number argument but none provided"),
                        ));
                    }
                }
                _ => {}
            }
        }
    }
    if pinger_args.timeout <= 0 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!(
                "Timeout must be greater than zero but {} passed",
                pinger_args.timeout
            ),
        ));
    }
    Ok(pinger_args)
}

pub(crate) fn pinger() -> Result<(), std::io::Error> {
    let prog_args: Vec<String> = std::env::args().collect();
    let args = match parse_args(&prog_args) {
        Ok(it) => it,
        Err(err) => {
            println!("{err}");
            return Err(err);
        }
    };

    let mut stream =
        match std::net::TcpStream::connect(format!("{}:{}", args.server_addr, args.server_port)) {
            Ok(it) => {
                it.set_read_timeout(Some(std::time::Duration::from_secs(args.timeout)))?;
                it
            }
            Err(err) => {
                println!(
                    "Unable to connect to {}:{} - {err}",
                    args.server_addr, args.server_port
                );
                return Err(err);
            }
        };

    Vec::<u8>::new()
        .write_varint(0x0)?
        .write_varint(0x0)?
        .write_str(args.server_addr)?
        .write_ushort(args.server_port)?
        .write_varint(0x01)?
        .send_to(&mut stream)?;

    Vec::<u8>::new().write_varint(0x00)?.send_to(&mut stream)?;

    stream.read_varint()?;
    stream.read_varint()?;

    let server_info_len = stream.read_varint()?;

    let mut server_info: Vec<u8> = vec![0u8; server_info_len as usize];
    stream.read_exact(&mut server_info)?;
    if args.verbose {
        println!("{:?}", String::from_utf8_lossy(server_info.as_ref()));
    }
    Ok(())
}
