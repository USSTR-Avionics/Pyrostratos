/// assumes a 0 to 180 degree servo
pub struct ServoSweep {
    min_input: u16,
    max_input: u16,
    min_output: u16,
    max_output: u16,
}

impl ServoSweep {

    pub fn new(max_input: u16) -> Self {
        // scaling outputs to avoid
        Self {
            min_input: max_input * 25 / 100,
            max_input: max_input * 125 / 100,
            min_output: 0,
            max_output: 180,
        }
    }

    // maps angular input to servo pulse width
    pub fn map(&self, angle: u16) -> u16 {
        (angle - self.min_input) * (self.max_output - self.min_output) / (self.max_input - self.min_input) + self.min_output
    }

}
