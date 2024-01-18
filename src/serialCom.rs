use std::sync::{Arc, RwLock};
use std::time::Duration;
use serialport::{DataBits, FlowControl, Parity, SerialPort, StopBits};

/**
 * SerialCom struct.
 */
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

/**
 * 串口状态结构体
 */
pub struct SerialStatus {
    pub port: Option<Box<dyn SerialPort>>, //串口打开对象附带错误处理
    pub connected: Arc<RwLock<bool>>, // 连接状态使用原子技术附加读写锁
    pub info: SerialCom, // 串口信息
}

impl SerialStatus {
    // 构建函数
    pub fn new(config: SerialCom) -> Self {
        SerialStatus {
            port: None,
            connected: Arc::new(RwLock::new(false)),
            info: config,
        }
    }

    // 连接方法
    pub fn connect(mut self) -> Result<Self, serialport::Error> {
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

    // 是否连接
    pub fn is_connected(&self) -> bool {
        if let Ok(connected) = self.connected.read() {
            *connected
        } else {
            false
        }
    }

    // 发送方法
    pub fn send(&self, data: &[u8]) -> Result<String, serialport::Error> {

        Ok(String::from("avv"))
    }

    // 断开方法
    pub fn disconnect(self) {
        if let Ok(mut connected) = self.connected.write() {
            if *connected {
                *connected = false;
            }
        }
    }
}