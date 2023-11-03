pub struct TemperatureSensor {
    value: f32,
    pin: u8,
    addr: u8,
}

impl TemperatureSensor {
    pub fn new(pin: u8, addr: u8) -> TemperatureSensor {
        TemperatureSensor {
            value: f32::NAN,
            pin,
            addr,
        }
    }

    pub fn read(&mut self) -> f32 {
        // read code
        self.value = 0.0;
        self.value
    }
}
