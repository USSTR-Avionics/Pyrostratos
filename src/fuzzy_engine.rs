const MAX_SETS: usize = 10;
const MAX_RULES: usize = 10;

use core::alloc::{GlobalAlloc, Layout};
use core::ptr::null_mut;

struct MyAllocator;

unsafe impl GlobalAlloc for MyAllocator {
    unsafe fn alloc(&self, _layout: Layout) -> *mut u8 { null_mut() }
    unsafe fn dealloc(&self, _ptr: *mut u8, _layout: Layout) {}
}

#[global_allocator]
static GLOBAL: MyAllocator = MyAllocator;


#[macro_export]
macro_rules! fuzzy_rule {
    ($engine:expr, IF $var:ident IS $set:ident THEN $conclusion:expr) => {
        $engine.rule($var.clone(), $set, $conclusion);
    };
}


#[derive(Copy, Clone)]
pub struct FuzzySet<'a> {
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

#[derive(Copy, Clone)]
pub struct FuzzyVariable<'a> {
    name: &'a str,
    sets: [FuzzySet<'a>; MAX_SETS],
    num_sets: usize,
}

impl<'a> FuzzyVariable<'a> {

    pub fn new(name: &str) -> FuzzyVariable {
        FuzzyVariable {
            name,
            sets: [FuzzySet::new("", |_| 0.0); MAX_SETS],
            num_sets: 0,
        }
    }

    // take in an array of any size and assign it to self.sets
    pub fn add_sets(&'a mut self, sets: &[FuzzySet<'a>; MAX_SETS]) {
        for i in 0..sets.len() {
            self.sets[i] = sets[i];
        }
        self.num_sets = sets.len();
    }
}

#[derive(Copy, Clone)]
pub struct FuzzyRule<'a> {
    variable: FuzzyVariable<'a>,
    set: FuzzySet<'a>,
    conclusion: f64,
}

#[derive(Copy, Clone)]
pub struct FuzzyEngine<'a> {
    rules: [FuzzyRule<'a>; MAX_RULES],
    num_rules: usize,
}

impl<'a> FuzzyEngine<'a> {

    pub fn new() -> FuzzyEngine<'a> {
        FuzzyEngine {
            rules: [FuzzyRule { 
                variable: FuzzyVariable::new(""), 
                set: FuzzySet::new("", |_| 0.0), 
                conclusion: 0.0 }; 
            MAX_RULES],
            num_rules: 0,
        }
    }

    pub fn rule(&mut self, variable: FuzzyVariable<'a>, set: FuzzySet<'a>, conclusion: f64) {
        if self.num_rules < MAX_RULES {
            self.rules[self.num_rules] = FuzzyRule {
                variable,
                set,
                conclusion,
            };
            self.num_rules += 1;
        }
    }

    pub fn infer(&self, input: &FuzzyVariable, input_variable: f64) -> f64 {
        let mut max_membership = 0.0;
        let mut conclusion = 0.0;

        for i in 0..self.num_rules {
            let rule = &self.rules[i];
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

