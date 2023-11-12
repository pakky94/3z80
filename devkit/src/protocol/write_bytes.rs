use crate::protocol::WRITE_BYTES;
use serialport::SerialPort;
use std::error::Error;
use std::io::{Read, Write};

pub fn write_bytes_to_addr(
    mut serial_port: &mut Box<dyn SerialPort>,
    addr_high: u16,
    data: &[u8],
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let addr_cmd = [
        WRITE_BYTES,
        (addr_high % 256) as u8,
        (addr_high / 256) as u8,
    ];
    serial_port.write(&addr_cmd)?;
    serial_port.write(data)?;

    let mut buf = [0u8; 1];
    serial_port.read_exact(&mut buf)?;

    if buf[0] == 'a' as u8 {
        Ok(())
    } else {
        Err("unexpected response".into())
    }
}
