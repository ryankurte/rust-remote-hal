use embedded_hal::digital;

use linux_embedded_hal::{Pin as PinDev};
use linux_embedded_hal::sysfs_gpio::Direction;

use crate::common::PinMode;
use crate::error::Error;

pub struct Pin {
    dev: PinDev,
}

impl Pin {
    pub fn new(path: &str, mode: PinMode) -> Result<Self, Error> {

        // Patch to export if pin follows a sensible layout
        let p = path.replace("/sys/class/gpio/gpio", "");
        if let Ok(id) = p.parse::<u64>() {
            let dev = PinDev::new(id);
            dev.export()?;
        }

        let dev = PinDev::from_path(path)
            .map_err(|e| Error::Remote(format!("{:?}", e)) )?;

        // export fails because you can't open the path before exporting...
        // docs recommend using pin by number...
        //dev.export()?;

        match mode {
            PinMode::Input => dev.set_direction(Direction::In)?,
            PinMode::Output => dev.set_direction(Direction::Out)?,
        }

        Ok(Self{dev})
    }
}

impl Drop for Pin {
    fn drop(&mut self) {
        // unexport disabled as export doesn't _really_ work
        self.dev.unexport().unwrap();
    }
}

impl digital::InputPin for Pin {
    //type Error = Error;

    fn is_high(&self) -> bool {
        self.dev.is_high()
    }

    fn is_low(&self) -> bool {
        self.dev.is_low()
    }
}

impl digital::OutputPin for Pin {
    //type Error = Error;

    fn set_high(&mut self) {
        self.dev.set_high();
    }

    fn set_low(&mut self) {
        self.dev.set_low();
    }
}


