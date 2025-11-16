mod pinger;
mod minecraft_packet;
mod converter;
mod minecraft_packet_impl;

use crate::pinger::pinger;
use std::process::exit;

fn main() {
    match pinger() {
        Ok(_) => exit(0),
        Err(_) => exit(-1),
    }
}
