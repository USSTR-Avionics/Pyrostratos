use core::result::Result;
use core::result::Result::{Ok, Err};
use super::error_t::GPIO_Error;



pub struct OxidizerValve {
    temperature: f32,
    pressure: f32,
    state: bool,
}

impl OxidizerValve {

    pub fn new() -> OxidizerValve {
        OxidizerValve {
            temperature: f32::NAN,
            pressure: f32::NAN,
            state: false,
        }
    }

    pub fn get_temperature(&self) -> Result<f32, GPIO_Error> {
        if self.temperature.is_nan() {
            Err(GPIO_Error::new("Temperature is not set", 0))
        } else {
            Ok(self.temperature)
        }
    }

    pub fn get_pressure(&self) -> Result<f32, GPIO_Error> {
        if self.pressure.is_nan() {
            Err(GPIO_Error::new("Pressure is not set", 0))
        } else {
            Ok(self.pressure)
        }
    }

}
