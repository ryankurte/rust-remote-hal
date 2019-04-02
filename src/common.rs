
use rand::random;
use structopt::StructOpt;
use hex;

use simple_error::SimpleError;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Request {
    pub id: u64,
    pub device: String,
    pub kind: RequestKind,
}

impl Request {
    pub fn new(device: String, kind: RequestKind) -> Self {
        Self{id: random(), device, kind}
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub enum RequestKind {
    #[structopt(name = "ping")]
    /// Send a ping message to the remote server
    Ping,

    #[structopt(name = "spi-connect")]
    /// Connect to the specified SPI device
    SpiConnect(SpiConnect),
    #[structopt(name = "spi-transfer")]
    /// Transfer data using a connected SPI device
    SpiTransfer{
        #[structopt(parse(try_from_str))]
        /// Data to be written in hexidecimal (ie. `0x112233` or `[00, 12, 01 a1]`)
        write_data: Data
    },
    #[structopt(name = "spi-write")]
    /// Write data using a connected SPI device
    SpiWrite{
        #[structopt(parse(try_from_str))]
        /// Data to be written in hexidecimal (ie. `0x112233` or `[00, 12, 01 a1]`)
        write_data: Data
    },
    #[structopt(name = "spi-disconnect")]
    /// Disconnect a connected SPI device
    SpiDisconnect,

    #[structopt(name = "pin-connect")]
    /// Connect to the specified pin
    PinConnect(PinMode),

    #[structopt(name = "pin-set")]
    /// Set the value of the specified pin
    PinSet(Value),
    #[structopt(name = "pin-get")]
    /// Fetch the value of the specified pin
    PinGet,
    #[structopt(name = "pin-disconnect")]
    /// Disconnect a connected pin
    PinDisconnect,

    #[structopt(name = "i2c-connect")]
    /// Connect to the specified I2C device
    I2cConnect,
    #[structopt(name = "i2c-write")]
    /// Write data to the provided address using a connected I2C device
    I2cWrite(I2cWrite),
    #[structopt(name = "i2c-read")]
    /// Read data from the provided address using a connected I2C device
    I2cRead(I2cRead),
    #[structopt(name = "i2c-write-read")]
    /// Write then read data from the provided address using a connected I2C device
    I2cWriteRead(I2cWriteRead),
    #[structopt(name = "i2c-disconnect")]
    /// Disconnect a connected I2C device
    I2cDisconnect,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub enum PinMode {
    #[structopt(name = "output")]
    /// Configure pin in output mode
    Output,
    #[structopt(name = "input")]
    /// Configure pin in input mode
    Input,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub enum SpiMode {
    #[structopt(name = "mode-0")]
    /// Configure SPI device in mode 0 (CPOL: 0, CPHA: 0)
    Mode0,
    #[structopt(name = "mode-1")]
    /// Configure SPI device in mode 1 (CPOL: 0, CPHA: 1)
    Mode1,
    #[structopt(name = "mode-2")]
    /// Configure SPI device in mode 2 (CPOL: 1, CPHA: 0)
    Mode2,
    #[structopt(name = "mode-3")]
    /// Configure SPI device in mode 3 (CPOL: 0, CPHA: 0)
    Mode3,
}

impl std::str::FromStr for SpiMode {
    type Err = SimpleError;

    fn from_str(mode: &str) -> Result<Self, Self::Err> {
        match mode {
            "0" => Ok(SpiMode::Mode0),
            "1" => Ok(SpiMode::Mode1),
            "2" => Ok(SpiMode::Mode2),
            "3" => Ok(SpiMode::Mode3),
            _ => Err(SimpleError::new("invalid spi mode")),
        }
    }
}

impl std::string::ToString for SpiMode {
    fn to_string(&self) -> String {
        match self {
            SpiMode::Mode0 => format!("0"),
            SpiMode::Mode1 => format!("1"),
            SpiMode::Mode2 => format!("2"),
            SpiMode::Mode3 => format!("3"),
        }
    }
}


#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Response {
    pub id: u64,
    pub kind: ResponseKind,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ResponseKind {
    Ok,
    Error(String),
    Unhandled,
    DeviceAlreadyBound,
    DeviceNotBound,

    SpiTransfer(Vec<u8>),
    PinGet(bool),
    I2cRead(Vec<u8>),
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Data {
    /// Data in hexadecimal form
    pub data: Vec<u8>,
}

impl std::str::FromStr for Data {
    type Err = hex::FromHexError;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        let s = s.replace(|v| v == ':' || v == ' ' || v == ',' || v == '[' || v == ']', "");
        let s = s.trim_start_matches("0x");
        let d = hex::decode(s)?;
        Ok(Data{data: d})
    }
}

impl std::string::ToString for Data {
    fn to_string(&self) -> String {
        format!("{:x?}", self.data)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct Value {
    pub value: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct SpiConnect {
    /// SPI baud rate in bps
    pub baud: u32,
    
    /// SPI mode
    pub mode: SpiMode,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct I2cWrite {
    /// I2C device address
    pub addr: u8,
    #[structopt(parse(try_from_str))]
    pub write_data: Data,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct I2cRead {
    /// I2C device address
    pub addr: u8,
    /// I2C read length
    pub read_len: u16,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct I2cWriteRead {
    /// I2C device address
    pub addr: u8,
    /// I2C read length
    pub read_len: u16,
    #[structopt(parse(try_from_str))]
    /// Data to be written in hexidecimal (ie. `0x112233` or `[00, 12, 01 a1]`)
    pub write_data: Data,
}

