pub struct GPIO_Error {
    message: &'static str,
    pin_number: u8,
}

impl GPIO_Error {
    pub fn new(message: &str, pin_number: u8) -> GPIO_Error {
        GPIO_Error {
            message,
            pin_number,
        }
    }
}

