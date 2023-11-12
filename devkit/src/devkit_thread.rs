use crate::protocol::write_bytes_to_addr;
use serialport::SerialPort;
use std::error::Error;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum DevkitCommand {
    ConnectSerial(String, u32),
    WriteBytes(u16, Vec<u8>),
    End,
}

struct State {
    serial_port: Option<Box<dyn SerialPort>>,
}

pub fn devkit_thread(
    rx: Receiver<DevkitCommand>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let mut state = State { serial_port: None };

    loop {
        match rx.try_recv() {
            Ok(DevkitCommand::ConnectSerial(port, rate)) => {
                state.serial_port = Some(
                    serialport::new(port.as_str(), rate)
                        .timeout(Duration::from_millis(10))
                        .open()
                        .expect("Failed to open port"),
                );
            }
            Ok(DevkitCommand::WriteBytes(addr, data)) => {
                match &mut state.serial_port {
                    Some(port) => write_bytes_to_addr(port, addr, data.as_slice())?,
                    None => {}
                }
                ()
            }
            Ok(DevkitCommand::End) => return Ok(()),
            Err(TryRecvError::Empty) => thread::sleep(Duration::from_millis(1)),
            Err(e) => return Err(e.into()),
        }
    }
}
