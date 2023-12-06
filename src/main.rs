#![no_main]
#![no_std]

extern crate panic_halt;

use hal::timer::{Channel, Tim2NoRemap};
// NOTE: required for linker script
#[allow(unused_imports)]
use stm32f1::stm32f103;

use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use nb::block;
use stm32f1xx_hal::{self as hal, pac, prelude::*, timer::Timer};

use pyrostratos::{
    embedded_allocator::init_allocator,
    fuzzy_engine::{FuzzyEngine, FuzzySet, FuzzyVariable},
};

#[entry]
fn setup() -> ! {
    init_allocator();
    main();
}

fn main() -> ! {
    // TODO: Add back in fuzzy logic
    /* Removing fuzzy logic for now, to test pws

    let mut engine = FuzzyEngine::new();

    let low = FuzzySet::new("Low", |x| x / 10.0);
    let medium = FuzzySet::new("Medium", |x| {
        let abs_val = if x < 0.0 { -x } else { x };
        1.0 - 2.0 * (abs_val - 5.0) / 10.0
    });
    let high = FuzzySet::new("High", |x| (10.0 - x) / 10.0);

    let mut temperature = FuzzyVariable::new("Temperature");
    temperature.add_set(&low);
    temperature.add_set(&medium);
    temperature.add_set(&high);

    engine.rule(temperature.clone(), low.clone(), 0.2);
    engine.rule(temperature.clone(), medium.clone(), 0.6);
    engine.rule(temperature.clone(), high.clone(), 0.9);

    // Test loop for changing values of inputs
    for input_temp in (0..=10).step_by(1) {
        let output = engine.infer(&temperature, input_temp as f64);
        hprintln!("Input temperature: {}, Output: {:.2}", input_temp, output);
    }

    */

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

    // Setup the alternate function I/O registers
    let mut afio = dp.AFIO.constrain();

    // Acquire the GPIOA peripheral
    let mut gpioa = dp.GPIOA.split();

    // Configure gpio A pin 5 as a push-pull output. The `crh` register is passed to the function
    // in order to configure the port. For pins 0-7, crl should be passed instead.
    let mut led = gpioa.pa5.into_push_pull_output(&mut gpioa.crl);

    // Configure the syst timer to trigger an update every second
    let mut timer = Timer::syst(cp.SYST, &clocks).counter_hz();
    timer.start(10.Hz()).unwrap();

    // TIM2
    let c1 = gpioa.pa0.into_alternate_push_pull(&mut gpioa.crl);
    let c2 = gpioa.pa1.into_alternate_push_pull(&mut gpioa.crl);
    let c3 = gpioa.pa2.into_alternate_push_pull(&mut gpioa.crl);

    let pins = (c1, c2, c3);

    let mut pwm = dp
        .TIM2
        .pwm_hz::<Tim2NoRemap, _, _>(pins, &mut afio.mapr, 10.kHz(), &clocks);

    // Enable clock each one of the channels
    pwm.enable(Channel::C1);
    pwm.enable(Channel::C2);
    pwm.enable(Channel::C3);

    let max = pwm.get_max_duty();

    // TODO: check the correct duty cycle for the servo
    pwm.set_duty(Channel::C3, max);

    // Extract the C3 channel from the pwm object
    let mut pwm_channel = pwm.split().2;

    // Use the PwmChannel object to set C3 to be full strength
    pwm_channel.set_duty(max);

    // Use the PwmChannel object to set C3 to be dim
    pwm_channel.set_duty(max / 4);

    // Use the PwmChannel object to set C3 to be zero
    pwm_channel.set_duty(0);

    // Wait for the timer to trigger an update and change the state of the LED
    loop {
        hprintln!("Hello, world!");
        block!(timer.wait()).unwrap();
        led.toggle();
        block!(timer.wait()).unwrap();
        led.toggle();
    }
}
