use std::io::{Read, Write};
use crate::converter::{int_to_varint, CONTINUE_BIT, SEGMENT_BITS};
use crate::minecraft_packet::{MinecraftPacketReader, MinecraftPacketWriter};

impl<'a> MinecraftPacketWriter<&'a mut Vec<u8>> for &'a mut Vec<u8> {
    fn write_varint(self, value: i32) -> Result<&'a mut Vec<u8>, std::io::Error> {
        let packet_value = int_to_varint(value);
        self.write_all(packet_value.as_ref())?;
        Ok(self)
    }

    fn write_ushort(self, value: u16) -> Result<&'a mut Vec<u8>, std::io::Error> {
        self.write_all(&value.to_be_bytes())?;
        Ok(self)
    }

    fn write_str(self, value: &str) -> Result<&'a mut Vec<u8>, std::io::Error> {
        let str_len = value.len();
        self.write_varint(str_len as i32)?;
        self.write_all(value.as_bytes())?;
        Ok(self)
    }

    fn as_packet(&mut self) -> Vec<u8> {
        let packet_len = self.len();
        let mut packet = Vec::<u8>::new();
        packet.extend_from_slice(&int_to_varint(packet_len as i32));
        packet.extend_from_slice(self);
        packet
    }

    fn send_to<W: Write>(mut self, stream: &mut W) -> Result<(), std::io::Error> {
        stream.write_all(self.as_packet().as_ref())?;
        Ok(())
    }
}

impl MinecraftPacketReader for &mut std::net::TcpStream {
    fn read_varint(self) -> Result<i32, std::io::Error> {
        let mut value: i32 = 0;
        for i in 0..5 {
            let mut one_byte_slice: [u8; 1] = [0; 1];
            self.read_exact(&mut one_byte_slice)?;
            let byte = one_byte_slice[0];
            value |= ((byte as i32) & (SEGMENT_BITS)) << (i * 7);
            if (byte as i32 & CONTINUE_BIT) == 0 {
                break;
            }
        }
        Ok(value)
    }
}
