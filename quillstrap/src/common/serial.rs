use crate::prelude::*;
use serialport::SerialPortInfo;

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
    debug!("Devices that were found automatically: {:?}", devices);
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
