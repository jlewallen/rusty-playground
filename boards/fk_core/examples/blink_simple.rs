#![no_std]
#![no_main]

extern crate cortex_m;
extern crate fk_core as hal;
extern crate panic_halt;

use hal::prelude::*;
use hal::{entry, CorePeripherals, Peripherals};
use hal::clock::GenericClockController;
use hal::delay::Delay;

#[entry]
fn main() -> ! {
    let mut peripherals = Peripherals::take().unwrap();
    let core = CorePeripherals::take().unwrap();
    let mut clocks = GenericClockController::with_external_32kosc(
        peripherals.GCLK,
        &mut peripherals.PM,
        &mut peripherals.SYSCTRL,
        &mut peripherals.NVMCTRL,
    );
    let mut pins = hal::Pins::new(peripherals.PORT);
    let mut red_led = pins.d13.into_open_drain_output(&mut pins.port);
    let mut delay = Delay::new(core.SYST, &mut clocks);
    loop {
        delay.delay_ms(100u16);
        red_led.set_high();
        delay.delay_ms(100u16);
        red_led.set_low();
    }
}
