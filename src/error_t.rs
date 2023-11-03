use crate::statemachine::StateMachine;

pub struct GPIO_Error {
    message: &'static str,
    pin_number: u8,
}

impl GPIO_Error {
    pub fn new(message: &'static str, pin_number: u8) -> GPIO_Error {
        GPIO_Error {
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
