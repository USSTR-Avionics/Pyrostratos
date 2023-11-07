#![no_main]
#![no_std]

extern crate panic_halt;

use nb::block;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f1::stm32f103;
use stm32f1xx_hal::{self as hal, pac, prelude::*, timer::Timer};

use pyrostratos::fuzzy_engine::{FuzzyEngine, FuzzySet, FuzzyVariable};



#[entry]
fn main() -> ! {
    
    let fuzzy_engine = setup_fuzzy_engine();
    let pressure_set = fuzzy_engine.get_inference_variable();
    let input_pressure = 45.0;
    let output_valve = fuzzy_engine.infer(&pressure_set, input_pressure);


    // Get access to the core peripherals from the cortex-m crate
    let cp = cortex_m::Peripherals::take().unwrap();
    // Get access to the device specific peripherals from the peripheral access crate
    let dp = pac::Peripherals::take().unwrap();

    // Take ownership over the raw flash and rcc devices and convert them into the corresponding
    // HAL structs
    let mut flash = dp.FLASH.constrain();
    let rcc = dp.RCC.constrain();

    // Freeze the configuration of all the clocks in the system and store the frozen frequencies in
    // `clocks`
    let clocks = rcc.cfgr.freeze(&mut flash.acr);

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Configure gpio A pin 5 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(10.Hz()).unwrap();

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        hprintln!("Hello, world!");
        block!(timer.wait()).unwrap();
        led.toggle();
        block!(timer.wait()).unwrap();
        led.toggle();
    }
}

fn setup_fuzzy_engine() -> FuzzyEngine<'static> {
    
    // Define the membership functions for the pressure settings
    let low_pressure = FuzzySet::new("low", |x| if x < 30.0 { 1.0 } else { 0.0 });
    let medium_pressure = FuzzySet::new("medium", |x| if x >= 30.0 && x <= 70.0 { 1.0 } else { 0.0 });
    let high_pressure = FuzzySet::new("high", |x| if x > 70.0 { 1.0 } else { 0.0 });

    // Create the fuzzy variables for pressure and valve
    let mut pressure = FuzzyVariable::new("pressure");

    // Create the fuzzy engine
    let mut engine = FuzzyEngine::new();

    let sets = [low_pressure, medium_pressure, high_pressure];
    let conclusion_sets = [25.0, 50.0, 100.0];

    engine.add_rules(&pressure, &sets, &conclusion_sets);
    pressure.add_sets(&[low_pressure, medium_pressure, high_pressure]);

    // Define the membership functions for the valve output settings
    let closed = FuzzySet::new("closed", |x| if x < 25.0 { 1.0 } else { 0.0 });
    let slightly_open = FuzzySet::new("slightly_open", |x| if x >= 25.0 && x <= 50.0 { 1.0 } else { 0.0 });
    let halfway_open = FuzzySet::new("halfway_open", |x| if x > 50.0 && x <= 75.0 { 1.0 } else { 0.0 });
    let fully_open = FuzzySet::new("fully_open", |x| if x > 75.0 { 1.0 } else { 0.0 });

    let mut valve = FuzzyVariable::new("valve");
    valve.add_sets(&[closed, slightly_open, halfway_open, fully_open]);

    engine
}

