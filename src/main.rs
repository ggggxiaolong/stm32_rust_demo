//! Blinks an LED
//!
//! This assumes that a LED is connected to pc13 as is the case on the blue pill board.
//!
//! Note: Without additional hardware, PC13 should not be used to drive an LED, see page 5.1.2 of
//! the reference manual for an explanation. This is not an issue on the blue pill.

#![deny(unsafe_code)]
#![no_std]
#![no_main]

mod blink;
mod read_gpio;

use panic_halt as _;

use cortex_m_rt::entry;

#[entry]
fn main() -> ! {
    read_gpio::read_gpio()
}