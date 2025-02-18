# 多平台串口库使用说明
基于serialport库实现
## 前置
```toml
[dependencies]
serialport = "4.2.2"
```
## 使用方法
### 在当前create引入模块
```rust
mod serial_com;
```
### 查询当前设备可用的串口
```rust
match serial_com::get_serial_port_available_list() {
    Ok(ports) => {
        for port in ports {
            println!("{:?}", port);
        }
    }
    Err(e) => {
        println!("{:?}", e);
    }
}
```
### 初始化初始化串口并连接
```rust
use std::sync::Arc;
use std::time::Duration;
use serialport::{DataBits, FlowControl, Parity, StopBits};

let connected_serial  = serial_com::SerialStatus::new(
    serial_com::SerialCom{
        port: Arc::from(String::from("COM9")),
        baud: 9_600,
        data_bits: DataBits::Eight,
        stop_bits: StopBits::One,
        parity: Parity::None,
        flow_control: FlowControl::None,
        timeout: Duration::from_millis(100),
    }
).connect();
```
### 对连接的串口进行错误处理以及连接成功后发送数据
#### 发送16进制数据并回调收到的16进制数据
```rust
match connected_serial {
    Ok(_) => {
        println!("Connected!");

        let mut command: Vec<u8> = vec![0x01, 0x03, 0x24, 0x02, 0x00, 0x02]; //复位
        let crc = calculate_crc(&command);
        command.push(crc[0]);
        command.push(crc[1]);

        match connected_serial.unwrap().send_receive_hex(&command) {
            Ok(buffer) => {
                println!("{:?}", &buffer[0..=7]);
                if &buffer[7..9] == calculate_crc(&buffer[0..=6]) {
                    let sv: f64 = i16::from_be_bytes([buffer[3], buffer[4]]) as f64 / 10.0;
                    let pv: f64 = i16::from_be_bytes([buffer[5], buffer[6]]) as f64 / 10.0;
                    println!("{:?}",[sv, pv]);
                }
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

// Modbus CRC-16 校验
fn calculate_crc(data: &[u8]) -> [u8; 2] {
    let mut crc: u16 = 0xFFFF;
    for &byte in data {
        crc ^= byte as u16;
        for _ in 0..8 {
            if crc & 0x0001 != 0 {
                crc >>= 1;
                crc ^= 0xA001;
            } else {
                crc >>= 1;
            }
        }
    }
    let crc_low_byte = (crc & 0xFF) as u8;
    let crc_high_byte = ((crc >> 8) & 0xFF) as u8;
    [crc_low_byte, crc_high_byte]
}
```
#### 使用字符串数据发送标准指令并接收回调的字符串数据
```rust
match connected_serial {
    Ok(_) => {
        println!("Connected!");
        match connected_serial.unwrap().send_receive_string(&String::from("*IDN?")) {
            Ok(buffer) => {
                println!("data receive\n{}", buffer);
            }
            Err(err) => {
                dbg!(err);
            }
        }
    }
    Err(err) => {
        dbg!(err);
    }
}
```