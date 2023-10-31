use core::result::Result::{Ok, Err};
use core::result::Result;
use super::error_t::StateError;



pub enum StateMachine {
    ActiveMonitor,
    FuelBurnStart,
    InjectionContinue,
    Purge,
    EmergencyStop,
}

pub struct StateMachineContext {
    state: StateMachine,
}

impl StateMachineContext {
    pub fn new() -> Self {
        StateMachineContext {
            state: StateMachine::ActiveMonitor,
        }
    }

    pub fn next(&mut self) -> Result<StateMachine, StateError> {
        self.state = match self.state {
            StateMachine::ActiveMonitor => StateMachine::FuelBurnStart,
            StateMachine::FuelBurnStart => StateMachine::InjectionContinue,
            StateMachine::InjectionContinue => 
                return Err(StateError::new("InjectionContinue state reached", StateMachine::InjectionContinue)),
            StateMachine::Purge => 
                return Err(StateError::new("Purge state reached", StateMachine::Purge)),
            StateMachine::EmergencyStop => 
                return Err(StateError::new("EmergencyStop state reached", StateMachine::EmergencyStop)),
        };
        Ok(self.state)
    }

    pub fn get_state(&self) -> StateMachine {
        self.state
    }

}

