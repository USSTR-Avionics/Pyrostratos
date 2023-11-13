use heapless::{String, Vec};

const MAX_RULES: usize = 10;

#[derive(Clone)]
pub struct FuzzySet{ 
    name: String<32>,
    membership_function: fn(f64) -> f64,
}

impl FuzzySet {
    pub fn new(name: &str, membership_function: fn(f64) -> f64) -> Self {
        FuzzySet {
            name: String::<32>::try_from(name).unwrap(),
            membership_function,
        }
    }

    pub fn membership(&self, value: f64) -> f64 {
        (self.membership_function)(value)
    }
}

#[derive(Clone)]
pub struct FuzzyVariable {
    name: String<32>,
    sets: Vec<FuzzySet, MAX_RULES>,
}

impl FuzzyVariable {
    pub fn new(name: &str) -> Self {
        FuzzyVariable {
            name: String::<32>::try_from(name).unwrap(), 
            sets: Vec::<FuzzySet, MAX_RULES>::new(),
        }
    }

    pub fn add_set(&mut self, set: &FuzzySet) {
        self.sets.push(set.clone());
    }
}

#[derive(Clone)]
pub struct Rule {
    variable: FuzzyVariable,
    set: FuzzySet,
    conclusion: f64,
}

pub struct FuzzyEngine {
    rules: Vec<Rule, MAX_RULES>,
}

impl FuzzyEngine {
    pub fn new() -> Self {
        FuzzyEngine {
            rules: Vec::<Rule, MAX_RULES>::new(),
        }
    }

    pub fn rule(&mut self, variable: FuzzyVariable, set: FuzzySet, conclusion: f64) {
        self.rules.push(Rule { variable, set, conclusion });
    }

    pub fn infer(&self, input: &FuzzyVariable, input_value: f64) -> f64 {
        let mut max_membership = 0.0;
        let mut conclusion = 0.0;

        for rule in &self.rules {
            if rule.variable.name == input.name {
                let membership = rule.set.membership(input_value);
                if membership > max_membership {
                    max_membership = membership;
                    conclusion = rule.conclusion;
                }
            }
        }

        conclusion
    }
}

