use std::sync::Arc;
use std::thread;
use std::time::Duration;
use serialport::{DataBits, FlowControl, Parity, StopBits};

mod serial_com;

fn main() {
    let connected_serial  = serial_com::SerialStatus::new(
        serial_com::SerialCom{
            port: Arc::from(String::from("COM2")),
            baud: 115200,
            data_bits: DataBits::Eight,
            stop_bits: StopBits::One,
            parity: Parity::None,
            flow_control: FlowControl::None,
            timeout: Duration::from_millis(100),
        }
    ).connect();

    match connected_serial {
        Ok(_) => {
            println!("Connected!");
            let test:[u8;1] = [0x55];
            match connected_serial.unwrap().send_receive(&test) {
                Ok(_) => {
                    println!("Send!");
                }
                Err(e) => {
                    dbg!(e);
                }
            }
        }
        Err(_) => {
            println!("Failed to connect!");
        }
    }


}


