struct FuzzySet {
    name: String,
    membership_function: Box<Fn(f64) -> f64>,
}

impl FuzzySet {

    pub fn new(name: String, membership_function: Box<Fn(f64) -> f64>) -> FuzzySet {
        FuzzySet {
            name: name,
            membership_function: membership_function,
        }
    }

    pub fn membership(&self, x: f64) -> f64 {
        (self.membership_function)(x)
    }
}

struct FuzzyVariable {
    name: String,
    sets: Vec<FuzzySet>,
}

impl FuzzyVariable {

    pub fn new(name: String, sets: Vec<FuzzySet>) -> FuzzyVariable {
        FuzzyVariable {
            name: name,
            sets: sets,
        }
    }

    pub fn add_set(&mut self, set: FuzzySet) {
        self.sets.push(set);
    }

}

struct FuzzyRule {
}



