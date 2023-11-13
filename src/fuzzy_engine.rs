use heapless::Vec;

const MAX_RULES: usize = 10;

#[derive(Clone)]
pub struct FuzzySet<'a> {
    name: &'a str,
    membership_function: fn(f64) -> f64,
}

impl <'a>FuzzySet<'a> {
    pub fn new(name: &'a str, membership_function: fn(f64) -> f64) -> Self {
        FuzzySet {
            name,
            membership_function,
        }
    }

    pub fn membership(&self, value: f64) -> f64 {
        (self.membership_function)(value)
    }
}

#[derive(Clone)]
pub struct FuzzyVariable<'a> {
    name: &'a str,
    sets: Vec<FuzzySet<'a>, MAX_RULES>,
}

impl <'a>FuzzyVariable<'a> {
    pub fn new(name: &'a str) -> Self {
        FuzzyVariable {
            name,
            sets: Vec::new(),
        }
    }

    pub fn add_set(&mut self, set: &'a FuzzySet) {
        self.sets.push(set.clone());
    }
}

#[derive(Clone)]
pub struct Rule<'a> {
    variable: FuzzyVariable<'a>,
    set: FuzzySet<'a>,
    conclusion: f64,
}

pub struct FuzzyEngine<'a> {
    rules: Vec<Rule<'a>, MAX_RULES>,
}

impl <'a>FuzzyEngine<'a> {
    pub fn new() -> Self {
        FuzzyEngine {
            rules: Vec::new(),
        }
    }

    pub fn rule(&'a mut self, variable: FuzzyVariable<'a>, set: FuzzySet<'a>, conclusion: f64) {
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

