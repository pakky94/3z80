mod write_bytes;
pub use write_bytes::write_bytes_to_addr;

const WRITE_BYTES: u8 = 'W' as u8;
const WRITE_SINGLE_BYTE: u8 = 'w' as u8;
