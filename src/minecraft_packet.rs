pub(crate) trait MinecraftPacketWriter<T> {
    fn write_varint(self, value: i32) -> Result<T, std::io::Error>;
    fn write_ushort(self, value: u16) -> Result<T, std::io::Error>;
    fn write_str(self, value: &str) -> Result<T, std::io::Error>;
    fn as_packet(&mut self) -> Vec<u8>;
    fn send_to<W: std::io::Write>(self, stream: &mut W) -> Result<(), std::io::Error>;
}

pub(crate) trait MinecraftPacketReader {
    fn read_varint(self) -> Result<i32, std::io::Error>;
}
