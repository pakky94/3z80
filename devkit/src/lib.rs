mod config;
mod devkit_thread;
mod protocol;

use crate::config::Config;
use crate::devkit_thread::DevkitCommand::ConnectSerial;
use crate::devkit_thread::{devkit_thread, DevkitCommand};
use serialport;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::sync::mpsc;
use std::sync::mpsc::Sender;
use std::thread;
use std::thread::JoinHandle;

pub fn run() {
    let (tx, rx) = mpsc::channel();
    let handle = thread::spawn(move || devkit_thread(rx));
    control_loop(handle, tx).expect("something broke :(");
}

fn control_loop(
    handle: JoinHandle<Result<(), Box<dyn Error + Send + Sync + 'static>>>,
    tx: Sender<DevkitCommand>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let config = Config::default();

    connect_port(&tx, &config)?;

    tx.send(DevkitCommand::End)?;
    handle.join().unwrap()
}

fn connect_port(
    tx: &Sender<DevkitCommand>,
    config: &Config,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let ports = serialport::available_ports().expect("No ports found!");
    for (i, p) in ports.iter().enumerate() {
        println!(" {}: {}", i, p.port_name);
    }

    let mut line = String::new();
    let stdin = std::io::stdin();
    stdin.lock().read_line(&mut line).unwrap();

    let idx: usize = line[..line.len() - 1].parse()?;

    tx.send(ConnectSerial(
        ports.get(idx).ok_or("port not found")?.port_name.clone(),
        config.serial_rate,
    ))?;

    Ok(())
}

#[derive(Debug)]
struct ThreadError {}

impl Display for ThreadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error joining thread")
    }
}

impl Error for ThreadError {}
