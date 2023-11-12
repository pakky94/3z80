use serialport::SerialPort;
use std::sync::mpsc::{Receiver, TryRecvError};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
pub enum DevkitCommand {
    ConnectSerial(String, u32),
    End,
}

struct State {
    serial_port: Option<Box<dyn SerialPort>>,
}

pub fn devkit_thread(rx: Receiver<DevkitCommand>) {
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
            Ok(DevkitCommand::End) => return,
            Err(TryRecvError::Empty) => thread::sleep(Duration::from_millis(1)),
            Err(_) => return,
        }
    }
}
