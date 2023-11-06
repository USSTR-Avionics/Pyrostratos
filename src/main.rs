#![no_main]
#![no_std]

extern crate panic_halt;

use nb::block;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use pyrostratos::fuzzy_engine;
use stm32f1::stm32f103;
use stm32f1xx_hal::{self as hal, pac, prelude::*, timer::Timer};


#[entry]
fn main() -> ! {
    
    let (mut fuzzy_engine, temperature) = setup_fuzzy_engine();
    fuzzy_engine.infer(temperature, 0.0 as f64);

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

fn setup_fuzzy_engine() -> FuzzyEngine {

    let mut engine = FuzzyEngine::new();

    let low = FuzzySet::new("Low", |x| x / 10.0);
    let medium = FuzzySet::new("Medium", |x| 1.0 - 2.0 * (x - 5.0).abs() / 10.0);
    let high = FuzzySet::new("High", |x| (10.0 - x) / 10.0);

    let mut temperature = FuzzyVariable::new("Temperature");
    temperature.add_set(&low);
    temperature.add_set(&medium);
    temperature.add_set(&high);

    fuzzy_rule!(engine, IF temperature IS low THEN 0.2);
    fuzzy_rule!(engine, IF temperature IS medium THEN 0.6);
    fuzzy_rule!(engine, IF temperature IS high THEN 0.9);

    engine
}

