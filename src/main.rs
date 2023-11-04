#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m::asm;
use cortex_m_rt::entry;
use cortex_m_semihosting::hprintln;
use stm32f3xx_hal::{self as hal, pac, prelude::*};
// use stm32f3::stm32f303;


#[entry]
fn main() -> ! {

    let dp = pac::Peripherals::take().unwrap();

    let mut rcc = dp.RCC.constrain();
    let mut gpiob = dp.GPIOB.split(&mut rcc.ahb);

    // led is connected to LD3 on the dev board
    // the pin is on PB_3
    let mut led = gpiob.pb3.into_push_pull_output(&mut gpiob.moder, &mut gpiob.otyper);

    loop {
        led.toggle().unwrap();
        for i in 0..1_000 {
            asm::nop();
        }
    }
}

