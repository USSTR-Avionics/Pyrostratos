extern crate alloc;
use alloc::vec::Vec;
use core::default::Default;
use core::result::Result::{Ok, Err};
use core::result::Result;

use crate::sensor_pressure::PressureSensor;
use crate::sensor_temperature::TemperatureSensor;
use super::error_t::GPIO_Error;



enum ValveState {
    Open,
    Closed,
}

pub struct OxidizerVentValve {
    temperature_sensors: Vec<TemperatureSensor>,
    temperature_threshold: f32,
    pressure_sensors: Vec<PressureSensor>,
    pressure_threshold: f32,
    state: ValveState,
}

impl OxidizerVentValve {

    pub fn new() -> OxidizerVentValve {
        OxidizerVentValve {
            temperature_sensors: Vec::new(),
            temperature_threshold: f32::default(),
            pressure_sensors: Vec::new(),
            pressure_threshold: f32::default(),
            state: ValveState::Closed,
        }
    }

    pub fn subscribe_temperature_sensor(&mut self, sensor: TemperatureSensor) {
        self.temperature_sensors.push(sensor);
    }

    pub fn get_temperature(&self) -> Result<f32, GPIO_Error> {
        Err(GPIO_Error::new("Temperature is not set", 0))
    }

    pub fn subscribe_pressure_sensor(&mut self, sensor: PressureSensor) {
        self.pressure_sensors.push(sensor);
    }

    pub fn get_pressure(&self) -> Result<f32, GPIO_Error> {
        Err(GPIO_Error::new("Pressure is not set", 0))
    }

    pub fn close_valve(&mut self) {
        self.state = ValveState::Closed;
        // close func
    }

    pub fn open_valve(&mut self) {
        self.state = ValveState::Open;
        // open func
    }

}

