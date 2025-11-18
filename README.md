# Minecraft Pinger

[![License: MIT](https://img.shields.io/badge/License-MIT-9d69d9.svg)](https://opensource.org/licenses/MIT)
[![Rust](https://img.shields.io/badge/rust-1.89.0%2B-orange.svg)](https://www.rust-lang.org/)
[![GitHub release](https://img.shields.io/github/v/release/Dreamfinity/minecraft-pinger)](https://github.com/Dreamfinity/minecraft-pinger/releases)
[![Build](https://github.com/Dreamfinity/minecraft-pinger/actions/workflows/release.yml/badge.svg)](https://github.com/Dreamfinity/minecraft-pinger/actions)
[![GitHub downloads](https://img.shields.io/github/downloads/Dreamfinity/minecraft-pinger/total)](https://github.com/Dreamfinity/minecraft-pinger/releases)
[![GitHub release date](https://img.shields.io/github/release-date/Dreamfinity/minecraft-pinger)](https://github.com/Dreamfinity/minecraft-pinger/releases)

A lightweight command-line tool written in Rust to ping Minecraft servers and retrieve their status information.

## What is Minecraft Pinger?

Minecraft Pinger is a simple utility that connects to Minecraft servers using the Server List Ping protocol to fetch
server information such as:

- Server description / MOTD (Message of the Day)
- Player count (online/max players)
- Server version
- Other metadata returned by the server

This tool implements the Minecraft protocol's handshake and status request packets to communicate with Minecraft
servers.

## Features

- ⚡ Blazingly fast — written in pure Rust with ZERO external dependencies
- 🎯 Simple command-line interface
- 🔌 Direct TCP connection to Minecraft servers
- 📦 No external dependencies required
- 🌐 Supports custom host and port configurations

## Server healthcheck usage

It can be easily used as server healthcheck - on success program prints server response and exit with 0 status. 
If error occured then program exit with `-1` (or `255`, depends on shell, OS, kernel - but mainly that it is non-zero) status

## Installation

### Download Pre-built Binaries (Recommended)

Download the latest release for your platform from
the [GitHub Releases](https://github.com/Dreamfinity/minecraft-pinger/releases) page.

#### Windows

- Download the latest release
- Extract `minecraft-pinger-{version}-x86_64-pc-windows-gnu.7z`
- Run `minecraft-pinger.exe`

#### Linux

```bash
# Download the latest release
wget https://github.com/Dreamfinity/minecraft-pinger/releases/latest/download/minecraft-pinger-{version}-x86_64-unknown-linux-gnu.tar.gz

# Extract the archive
tar -xzf minecraft-pinger-{version}-x86_64-unknown-linux-gnu.tar.gz

# Make it executable (if needed)
chmod +x minecraft-pinger-{version}-x86_64-unknown-linux-gnu/minecraft-pinger

# Optionally, move to a directory in your PATH
sudo mv minecraft-pinger-{version}-x86_64-unknown-linux-gnu/minecraft-pinger /usr/local/bin/
```

#### macOS

```bash
# Download the latest release
wget https://github.com/Dreamfinity/minecraft-pinger/releases/latest/download/minecraft-pinger-{version}-x86_64-apple-darwin.tar.gz

# Extract the archive
tar -xzf minecraft-pinger-{version}-x86_64-apple-darwin.tar.gz

# Make it executable (if needed)
chmod +x minecraft-pinger-{version}-x86_64-apple-darwin/minecraft-pinger

# Optionally, move to a directory in your PATH
sudo mv minecraft-pinger-{version}-x86_64-apple-darwin/minecraft-pinger /usr/local/bin/
```

### Build from Source

#### Prerequisites

- Rust 1.89.0 or later
- Cargo (comes with Rust)

1. Clone the repository:

    ```bash
    git clone https://github.com/Dreamfinity/minecraft-pinger.git
    cd minecraft-pinger
    ```

2. Build the project:

    ```bash
    cargo build --release
    ```

3. The executable will be available at `target/release/minecraft-pinger`

### Quick Installation from Source

```bash
cargo install --path .
```

## Usage

You can use flag `--print` (or `-p`) to print server response. Without it program just exit with proper status code as
described above

### Basic Usage

Ping a local server (defaults to `127.0.0.1:25565`):

```bash
minecraft-pinger
```

### Ping a Specific Server

```bash
minecraft-pinger example.com:25565
```

### Examples

Ping a server with default port (25565):

```bash
minecraft-pinger mc.hypixel.net
```

Ping a server with custom port:

```bash
minecraft-pinger myserver.com:25566
```

Ping localhost:

```bash
minecraft-pinger localhost:25565
```

## Output

The tool outputs the raw JSON response from the Minecraft server, which typically includes:

```json
{
  "version": {
    "name": "1.20.1",
    "protocol": 763
  },
  "players": {
    "max": 100,
    "online": 5
  },
  "description": {
    "text": "A Minecraft Server"
  }
}
```

## How It Works

The tool implements the Minecraft Server List Ping protocol:

1. **Handshake**: Sends a handshake packet (0x00) with the server address and port
2. **Status Request**: Sends a status request packet (0x00) to request server information
3. **Status Response**: Reads and parses the JSON response from the server

The implementation uses custom packet readers and writers to handle Minecraft's VarInt encoding and packet structure.

## Technical Details

- **Protocol Version**: Status protocol (Next State = 1)
- **Connection**: Direct TCP socket connection
- **Data Format**: Minecraft protocol packets with VarInt encoding
- **Response Format**: UTF-8 encoded JSON

## Project Structure

```
minecraft-pinger/
├── src/
│   ├── main.rs                    # Entry point
│   ├── pinger.rs                  # Main pinger logic
│   ├── minecraft_packet.rs        # Packet reader/writer traits
│   ├── minecraft_packet_impl.rs   # Trait implementations
│   └── converter.rs               # Data conversion utilities
├── Cargo.toml
└── README.md
```

## Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

## License

Code is distributed under the MIT license

## Acknowledgments

- Based on the [Minecraft Protocol](https://minecraft.wiki/w/Java_Edition_protocol#Protocol) specification
- Inspired by the need for a simple, dependency-free Minecraft server pinger

## Troubleshooting

### Connection Refused

- Ensure the Minecraft server is running
- Check that the server address and port are correct
- Verify that the server allows status requests

### Invalid Response

- Some servers may have query disabled
- Ensure you're using the correct port (default is `25565`)
- The server must be a Java Edition Minecraft server (this tool doesn't support Bedrock Edition)

## Future Enhancements

- [ ] Pretty-print JSON output
- [ ] Colored terminal output
- [ ] Support for server icons
- [ ] Ping latency measurement
- [ ] Multiple server batch pinging
- [ ] JSON/YAML output formats

## Related Projects

- [minecraft.wiki](https://minecraft.wiki/w/Java_Edition_protocol#Protocol) - Minecraft Protocol Documentation

## Author

[Georgii <Veritaris> Imeshkenov](https://github.com/Veritaris)

## Support

If you encounter any issues or have questions,
please [open an issue](https://github.com/Dreamfinity/minecraft-pinger/issues) on GitHub.
