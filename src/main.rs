#![no_main]
#![no_std]

extern crate panic_halt;

use hal::timer::Pwm;
use nb::block;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
// required for linker script
use stm32f1::stm32f103::{self, tim2};
use stm32f1xx_hal::{self as hal, pac, prelude::*, timer::Timer, timer::pwm};

use pyrostratos::{fuzzy_engine::{FuzzyEngine, FuzzyVariable, FuzzySet}, embedded_allocator::init_allocator};


#[entry]
fn setup() -> ! {
    init_allocator();
    main();
}

fn main() -> ! {
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

    // Configure servo PWM pins
    let p = 

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

