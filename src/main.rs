#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::prelude::*;
use esp_println::{logger::init_logger_from_env, println};

#[embassy_executor::task]
async fn task() {
    log::info!("Task");
    println!("Taks")
}

#[esp_hal::entry]
fn main() -> ! {
    init_logger_from_env();

    let peripherals = esp_hal::peripherals::Peripherals::take();
    let system = peripherals.SYSTEM.split();
    let clocks = esp_hal::clock::ClockControl::boot_defaults(system.clock_control).freeze();
    let delay = esp_hal::delay::Delay::new(&clocks);

    log::info!("Start");
    loop {
        log::info!("Running");
        delay.delay_millis(1000);
    }
}
