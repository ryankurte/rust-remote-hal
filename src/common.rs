
use rand::random;
use structopt::StructOpt;

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
    Ping,

    #[structopt(name = "spi-connect")]
    SpiConnect(SpiConnect),
    #[structopt(name = "spi-transfer")]
    SpiTransfer(Data),
    #[structopt(name = "spi-write")]
    SpiWrite(Data),
    #[structopt(name = "spi-disconnect")]
    SpiDisconnect,

    #[structopt(name = "pin-connect")]
    PinConnect,
    #[structopt(name = "pin-set")]
    PinSet(Value),
    #[structopt(name = "pin-get")]
    PinGet,
    #[structopt(name = "pin-disconnect")]
    PinDisconnect,

    #[structopt(name = "i2c-connect")]
    I2cConnect,
    #[structopt(name = "i2c-write")]
    I2cWrite(I2cWrite),
    #[structopt(name = "i2c-read")]
    I2cRead(I2cRead),
    #[structopt(name = "i2c-writeRead")]
    I2cWriteRead(I2cWriteRead),
    #[structopt(name = "i2c-disconnect")]
    I2cDisconnect,
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
    pub data: Vec<u8>,
}

impl std::str::FromStr for Data {
    type Err = ();

    fn from_str(s: &str) -> std::result::Result<Self, ()> {
        Err(())
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
    pub mode: u32,
}

#[derive(Debug, Clone, Serialize, Deserialize, StructOpt)]
pub struct I2cWrite {
    /// I2C device address
    pub addr: u8,
    #[structopt(flatten)]
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
    #[structopt(flatten)]
    pub write_data: Data,
    /// I2C read length
    pub read_len: u16,
}

