use crate::statemachine::StateMachine;

pub struct GpioError {
    message: &'static str,
    pin_number: u8,
}

impl GpioError {
    pub fn new(message: &'static str, pin_number: u8) -> GpioError {
        GpioError {
            message,
            pin_number,
        }
    }
}

pub struct StateError {
    message: &'static str,
    curr_state: StateMachine,
}

impl StateError {
    pub fn new(message: &'static str, curr_state: StateMachine) -> StateError {
        StateError {
            message,
            curr_state,
        }
    }
}

