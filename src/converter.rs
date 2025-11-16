use std::io::Write;

pub(crate) const SEGMENT_BITS: i32 = 0b01111111;
pub(crate) const CONTINUE_BIT: i32 = 0b10000000;

pub(crate) fn int_to_varint(value: i32) -> Vec<u8> {
    let mut value = value;
    let mut rb;
    let mut buffer = Vec::<u8>::new();
    while (value & CONTINUE_BIT) > 0 {
        let data_to_write: u8 = (value & SEGMENT_BITS) as u8;
        value >>= 7;
        rb = buffer.write(&[data_to_write]).unwrap();
        println!("wrote {rb} bytes");
    }
    let val = [(value & SEGMENT_BITS) as u8];
    buffer.write_all(&val).unwrap();
    buffer
}
