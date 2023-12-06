pub struct ServoSweep {
    min_input: u32,
    max_input: u32,
    min_output: u32,
    max_output: u32,
}

impl ServoSweep {

    pub fn new(max_input: u32) -> Self {
        // scaling outputs to avoid
        Self {
            min_input: max_input * 25 / 100,
            max_input: max_input * 125 / 100,
            min_output: 0,
            max_output: 180,
        }
    }

    // maps angular input to servo pulse width
    pub fn map(&self, angle: u32) -> u32 {
        (angle - self.min_input) * (self.max_output - self.min_output) / (self.max_input - self.min_input) + self.min_output
    }

}
