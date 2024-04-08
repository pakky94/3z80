mod config;
mod devkit_thread;
mod protocol;

use crate::config::Config;
use crate::devkit_thread::DevkitCommand::ConnectSerial;
use crate::devkit_thread::{devkit_thread, DevkitCommand, DevkitResponse};
use serialport;
use std::error::Error;
use std::fmt::{Display, Formatter};
use std::io::BufRead;
use std::sync::mpsc;
use std::sync::mpsc::{Receiver, Sender};
use std::thread;
use std::thread::JoinHandle;
use std::time::Duration;

pub fn run() {
    let (tx, rx) = mpsc::channel();
    let (res_tx, res_rx) = mpsc::channel();
    let handle = thread::spawn(move || devkit_thread(rx, res_tx));
    control_loop(handle, tx, res_rx).expect("something broke :(");
}

fn control_loop(
    handle: JoinHandle<Result<(), Box<dyn Error + Send + Sync + 'static>>>,
    tx: Sender<DevkitCommand>,
    res: Receiver<DevkitResponse>,
) -> Result<(), Box<dyn Error + Send + Sync + 'static>> {
    let config = Config::default();

    connect_port(&tx, &config)?;

    let mut current_mem = vec![];
    let stdin = std::io::stdin();

    loop {
        let mut cmd = String::new();
        stdin.lock().read_line(&mut cmd)?;
        let args = cmd
            .split(" ")
            .into_iter()
            .map(|a| a.trim())
            .filter(|a| !a.is_empty())
            .collect::<Vec<_>>();

        if args.len() == 0 {
            println!("helpful message here :D");
            continue
        }

        match args[0] {
            "u" => {
                let filename = *args.get(1).unwrap_or(&"data");
                println!("opening: {}", filename);
                if let Ok(data) = std::fs::read(filename) {
                    current_mem = update_memory(data, current_mem, &tx, &res)?;
                } else {
                    println!("unable to read file: {}", filename);
                }
            }
            "q" => {
                break;
            }
            &_ => {}
        }
    }

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
    stdin.lock().read_line(&mut line)?;

    let idx: usize = line[..line.len() - 1].parse()?;

    tx.send(ConnectSerial(
        ports.get(idx).ok_or("port not found")?.port_name.clone(),
        config.serial_rate,
    ))?;

    Ok(())
}

fn update_memory(
    mut target: Vec<u8>,
    actual: Vec<u8>,
    tx: &Sender<DevkitCommand>,
    res: &Receiver<DevkitResponse>,
) -> Result<Vec<u8>, Box<dyn Error + Send + Sync + 'static>> {
    // pad target mem to multiple of 256
    loop {
        if target.len() % 256 == 0 {
            break;
        }
        target.push(0);
    }

    let mut i = 0;
    let mut blocks = vec![];

    loop {
        if i >= target.len() {
            break;
        }

        if i + 256 > actual.len() || actual[i..i + 256] != target[i..i + 256] {
            blocks.push(((i / 256) as u16, target[i..i + 256].to_vec()));
        }

        i += 256;
    }

    let blocks_to_update = blocks.len();
    let pad_len = blocks_to_update.to_string().chars().count();

    for (i, (addr, data)) in blocks.into_iter().enumerate() {
        println!(
            "{:0width$}/{:0width$}",
            i + 1,
            blocks_to_update,
            width = pad_len
        );

        tx.send(DevkitCommand::WriteBytes(addr, data))?;

        let r = res.recv_timeout(Duration::from_millis(1000))?;
        assert_eq!(r, DevkitResponse::Done);
    }

    Ok(target)
}

#[derive(Debug)]
struct ThreadError {}

impl Display for ThreadError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "error joining thread")
    }
}

impl Error for ThreadError {}
