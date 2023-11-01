pub struct OxidizerFuzzyInputs {
    low_pressure: f32,
    medium_pressure: f32,
    high_pressure: f32,
}

impl OxidizerFuzzyInputs {

    pub fn new(low_pressure: f32, medium_pressure: f32, high_pressure: f32) -> OxidizerFuzzyInputs {
        OxidizerFuzzyInputs {
            low_pressure,
            medium_pressure,
            high_pressure,
        }
    }

    fn membership(&self, x: f32) -> f32 {
        if x <= self.low_pressure || x >= self.high_pressure {
            0.0
        } else if x < self.medium_pressure {
            (x - self.low_pressure) / (self.medium_pressure - self.low_pressure)
        } else {
            (self.high_pressure - x) / (self.high_pressure - self.medium_pressure)
        }
    }

    fn fuzzy_rule(&self, pressure_valve_input: f32) -> bool {
    
        let pressure_membership = self.membership(pressure_valve_input);
    
        if pressure_membership > 0.0 {
            true
        } else if pressure_membership < 0.3 {
            false
        } else {
            false
        }
    
    }

    pub fn get_valve_action(&self, pressure_valve_input: f32) -> bool {
        self.fuzzy_rule(pressure_valve_input)
    }
    
}

