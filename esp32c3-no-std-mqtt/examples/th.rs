//! Run this:
//!
//! 1. Set SSID and PASSWORD environment variables to match with the WiFi connection you're using
//!   - `export SSID=my_wifi PASSWORD=my_pass`
//!   - Note that your wifi password is now stored in your environment.
//! 2.
#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

// peripherals-related imports
use hal::{
    clock::{ClockControl, CpuClock},
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    Delay, IO,
};

use esp_backtrace as _;
use esp_println::println;

use shtcx::{self, LowPower, PowerMode};

#[entry]
fn main() -> ! {
    let peripherals = Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = ClockControl::configure(system.clock_control, CpuClock::Clock160MHz).freeze();

    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    let i2c0 = I2C::new(
        peripherals.I2C0,
        io.pins.gpio8,
        io.pins.gpio10,
        400u32.kHz(),
        &clocks,
    );

    let mut sht = shtcx::shtc3(i2c0);
    let mut delay = Delay::new(&clocks);

    sht.wakeup(&mut delay).unwrap();

    loop {
        let measurement = sht.measure(PowerMode::NormalMode, &mut delay).unwrap();

        println!("{measurement:?}");
    }
}
