use std::{time::Duration, u8};

use crate::prelude::*;
use serialport::{SerialPort, SerialPortInfo};

const DEFAULT_BAUDRATE: u32 = 1500000;

pub fn get_serial_devices_native() -> Vec<SerialPortInfo> {
    let mut ports2 = Vec::new();
    let ports = serialport::available_ports().expect("Failed to get serial ports");
    for port in ports.iter().clone() {
        match &port.port_type {
            serialport::SerialPortType::UsbPort(_) => ports2.push(port.clone()),
            _ => {}
        }
    }
    ports2
}

pub fn get_serial_devices_native_names() -> Vec<String> {
    let mut serials_clean = Vec::new();
    let serials = get_serial_devices_native();
    for serial in serials {
        serials_clean.push(serial.port_name);
    }
    serials_clean
}

pub fn get_serial_devices() -> Vec<String> {
    let mut serials: Vec<String> = Vec::new();
    let ports = get_serial_devices_native();
    for port in ports {
        match port.port_type {
            serialport::SerialPortType::UsbPort(usb_port_info) => {
                let man = usb_port_info.manufacturer.unwrap_or_default();
                let dev = usb_port_info.product.unwrap_or_default();
                if man == "QinHeng Electronics" || dev == "CH340 serial converter" {
                    serials.push(port.port_name);
                }
            }
            _ => {}
        }
    }
    serials
}

pub fn choose_serial_port() -> String {
    let devices = get_serial_devices();
    let devices_clean = get_serial_devices_native_names();

    if devices_clean.is_empty() {
        warn!("No serial device found!");
        return "".to_string();
    }

    debug!("Devices that were found automatically: {:?}", devices);
    #[allow(unused_assignments)]
    let mut input = String::new();

    if devices.iter().count() == 1 {
        let device = devices[0].clone();
        info!(
            "Only one serial device matches our criteria, choosing it: {}",
            device
        );
        return device;
    } else if devices.iter().count() > 1 {
        input = devices_clean[Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "There are more serial devices available, choose one (port_name):\n{:#?}\n",
                get_serial_devices_native()
            ))
            .default(0)
            .items(&devices_clean)
            .interact()
            .unwrap()]
        .clone();

        if !devices.contains(&input) {
            warn!("No such device! But let's try it anyway, I trust you bro");
        }
    } else {
        input = devices_clean[Select::with_theme(&ColorfulTheme::default())
            .with_prompt(format!(
                "Auto detection failed, choose one (port_name):\n{:#?}\n",
                get_serial_devices_native()
            ))
            .default(0)
            .items(&devices_clean)
            .interact()
            .unwrap()]
        .clone();
    }

    input
}

pub fn open_port(port: String) -> Box<dyn SerialPort> {
    serialport::new(port.clone(), DEFAULT_BAUDRATE)
        .timeout(Duration::from_millis(100))
        .open()
        .expect(&format!("Failed to open port: {}", port))
}

// No \n added at the end!
pub fn send_serial_message(port: String, str: &str) {
    let mut port_open = open_port(port.clone());
    port_open
        .write(str.as_bytes())
        .expect(&format!("Failed to write to port: {}", port));
}

pub fn send_serial_ascii(port: String, ascii: u8) {
    let mut port_open = open_port(port.clone());
    port_open
        .write(&[ascii])
        .expect(&format!("Failed to write to port: {}", port));
}

pub fn read_serial(port: String, slice: &mut [u8]) {
    let mut port_open = open_port(port.clone());
    if let Err(err) = port_open.read(slice) {
        warn!("Serial read error: {:?}", err);
    }
}

use std::io::{self, Read};
use std::sync::{mpsc, Arc};
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::{Instant};

pub fn send_read_serial(port: String, s: &str) -> String {
    let (tx, rx) = mpsc::channel::<Vec<u8>>();
    let stop = Arc::new(AtomicBool::new(false));
    let stop_reader = stop.clone();
    let port_clone = port.clone();

    let reader = thread::spawn(move || {
        let mut sp = match serialport::new(port_clone, DEFAULT_BAUDRATE)
            .timeout(Duration::from_millis(50))
            .open()
        {
            Ok(p) => p,
            Err(_) => return,
        };
        let mut buf = [0u8; 1024];
        while !stop_reader.load(Ordering::Relaxed) {
            match sp.read(&mut buf) {
                Ok(n) if n > 0 => { let _ = tx.send(buf[..n].to_vec()); }
                Err(ref e) if e.kind() == io::ErrorKind::TimedOut => {}
                Err(_) => break,
                _ => {}
            }
        }
    });

    thread::sleep(Duration::from_millis(100));
    send_serial_message(port.clone(), &format!("{}\n\r", s));

    let mut out: Vec<u8> = Vec::new();
    let idle = Duration::from_millis(500);
    let overall = Duration::from_secs(5);
    let start = Instant::now();
    let mut last = Instant::now();
    loop {
        if Instant::now().duration_since(start) >= overall { break; }
        match rx.recv_timeout(Duration::from_millis(50)) {
            Ok(chunk) => { out.extend_from_slice(&chunk); last = Instant::now(); }
            Err(mpsc::RecvTimeoutError::Timeout) => {
                if Instant::now().duration_since(last) >= idle { break; }
            }
            Err(mpsc::RecvTimeoutError::Disconnected) => break,
        }
    }

    stop.store(true, Ordering::Relaxed);
    let _ = reader.join();

    let s = String::from_utf8_lossy(&out).to_string();
    info!("Received from serial: {}", s);
    s
}