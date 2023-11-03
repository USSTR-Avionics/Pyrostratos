#![no_main]
#![no_std]

extern crate panic_halt;

use cortex_m_rt::entry;
use cortex_m_semihosting::{debug, hprintln};
// ! needs to be imported for the linker to find interrupt vectors
use stm32f3::stm32f303;

#[entry]
fn main() -> ! {
    loop {
        hprintln!("Hello, World!");
    }
}

