use std::io::Read;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use serialport::{DataBits, Error, FlowControl, Parity, SerialPort, StopBits};

/// SerialCom struct.
#[derive(Clone, Debug)]
pub struct SerialCom {
    pub port: Arc<String>, // 串口号
    pub baud: u32, // 波特率
    pub data_bits: DataBits, // 数据位
    pub stop_bits: StopBits, // 停止位
    pub parity: Parity, // 校验位
    pub flow_control: FlowControl, // 流控
    pub timeout: Duration, // 超时时间
}

/// 串口状态结构体
pub struct SerialStatus {
    pub port: Option<Box<dyn SerialPort>>, // 串口打开对象附带错误处理
    pub connected: Arc<RwLock<bool>>, // 连接状态使用原子技术附加读写锁
    pub info: SerialCom, // 串口信息
}

impl SerialStatus {
    /// 构建函数
    pub fn new(config: SerialCom) -> Self {
        SerialStatus {
            port: None,
            connected: Arc::new(RwLock::new(false)),
            info: config,
        }
    }

    /// 连接到串行端口。
    /// # Arguments
    /// * `self` - 对 `SerialStatus` 结构的引用。
    /// # Returns
    /// 包含 `SerialStatus` 结构的 `Result` ，其中包含连接的串行端口或错误。
    pub fn connect(mut self) -> Result<Self, Error> {
        let port = serialport::new(self.info.port.as_str(), self.info.baud)
            .data_bits(self.info.data_bits)
            .stop_bits(self.info.stop_bits)
            .parity(self.info.parity)
            .flow_control(self.info.flow_control)
            .timeout(self.info.timeout)
            .open()?;
        self.port = port.try_clone().ok();
        if let Ok(mut connected) = self.connected.write() {
            *connected = true;
        }
        Ok(self)
    }

    /// 是否连接
    pub fn is_connected(&self) -> bool {
        if let Ok(connected) = self.connected.read() {
            *connected
        } else {
            false
        }
    }

    /// 以十六进制格式向 modbus 设备发送数据并回读响应。
    /// ## 适用 欧姆龙 岛电 宇电 PID温控表 以及 MFC气体流量计
    /// # Arguments
    /// * `data` - 要以十六进制格式发送到 modbus 设备的数据。
    /// # Returns
    /// 一个 `Result<[u8; 32], Error>` ，包含包含来自 modbus 设备的响应或错误的字节片。
    pub fn send_receive_hex(&self, data: &[u8]) -> Result<[u8; 32], Error> {
        let mut port = self.port.as_ref().ok_or(Error::new(
            serialport::ErrorKind::NoDevice,
            "No port available",
        ))?
        .try_clone()?;
        let mut buffer: [u8; 32] = [0; 32];
        port.write(data)?;
        port.read(&mut buffer)?;
        Ok(buffer)
    }

    /// 将字符串发送到连接的设备，并以字符串形式返回响应。
    /// ## 适用 KEITHLEY 同惠 安捷伦 等nV nA源表
    /// # Arguments
    ///
    /// * `data` - 要发送到设备的字符串。
    ///
    /// # Returns
    ///
    /// `Result<String, Error>` ，包含包含来自设备的响应或错误的字节片。
    pub fn send_receive_string(&self, data: &String) -> Result<String, Error> {
        let mut port = self.port.as_ref().ok_or(Error::new(
            serialport::ErrorKind::NoDevice,
            "No port available",
        ))?
            .try_clone()?;
        let mut serial_buf: Vec<u8> = vec![0; 128];
        let data_with_newline = format!("{}\r\n", data); // 在字符串末尾添加换行符
        port.write(data_with_newline.as_bytes())?;
        let bytes_read = port.read(serial_buf.as_mut_slice())?;
        serial_buf.truncate(bytes_read);
        serial_buf.pop(); //    remove  \r
        serial_buf.pop(); //    remove  \n
        match String::from_utf8_lossy(&serial_buf).parse() {
            Ok(result) => Ok(result),
            Err(_) => Err(Error::new(
                serialport::ErrorKind::InvalidInput,
                "Failed to parse received data",
            )),
        }
    }

    /// 断开方法
    pub fn disconnect(self) {
        if let Ok(mut connected) = self.connected.write() {
            if *connected {
                *connected = false;
            }
        }
    }
}
