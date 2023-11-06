struct FuzzySet<'a> {
    name: &'a str,
    member_func: fn(f64) -> f64,
}

impl FuzzySet<'_> {

    pub fn new(name: &str, member_func: fn(f64) -> f64) -> FuzzySet {
        FuzzySet {
            name,
            member_func,
        }
    }

    pub fn membership(&self, value: f64) -> f64 {
        (self.member_func)(value)
    }
}


struct FuzzyVariable<'a> {
    name: &'a str,
    sets: Vec<FuzzySet>,
}

impl FuzzyVariable<'_> {

    pub fn new(name: &str) -> FuzzyVariable {
        FuzzyVariable {
            name,
            sets: Vec::new(),
        }
    }
    
    pub fn add_set(&mut self, set: FuzzySet) {
        self.sets.push(set);
    }
}

struct FuzzyRule<'a> {
    variable: FuzzyVariable<'a>,
    set: FuzzySet<'a>,
    conclusion: f64,
}

struct FuzzyEngine<'a> {
    rules: Vec<FuzzyRule>,
}

impl FuzzyEngine<'static> {

    pub fn new() -> FuzzyEngine {
        FuzzyEngine {
            rules: Vec::new(),
        }
    }

    pub fn rule(&mut self, variable: FuzzyVariable, set: FuzzySet, conclusion: f64) {
        self.rules.push(FuzzyRule {
            variable,
            set,
            conclusion,
        });
    }

    pub fn infer(&self, input: &FuzzyVariable, input_variable: f64) -> f64 {
        let mut max_membership = 0.0;
        let mut conclusion = 0.0;

        for rule in &self.rules {
            if rule.variable.name == input.name {
                let membership = rule.set.membership(input_variable);
                if membership > max_membership {
                    max_membership = membership;
                    conclusion = rule.conclusion;
                }
            }
        }
        conclusion
    }
}

