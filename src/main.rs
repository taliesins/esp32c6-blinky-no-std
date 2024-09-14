#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::{
    delay::Delay,
    gpio::{Io, Level, Output},
    prelude::*,
};
use esp_println::{logger::init_logger_from_env, println};

#[embassy_executor::task]
async fn test_task() {
    log::info!("Task");
    println!("Taks")
}

#[main]
async fn main(_s: embassy_executor::Spawner) {
    init_logger_from_env();

    let peripherals = esp_hal::peripherals::Peripherals::take();
    //let peripherals = esp_hal::init(esp_hal::Config::default()); //doesn't work ;<
    let system = esp_hal::system::SystemControl::new(peripherals.SYSTEM);
    let clocks = esp_hal::clock::ClockControl::max(system.clock_control).freeze();
    let delay = Delay::new(&clocks);

    // Set GPIO0 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio0, Level::High);

    log::info!("Start");

    _s.spawn(test_task()).ok();

    loop {
        log::info!("Running");
        led.toggle();
        delay.delay_millis(500);
        led.toggle();
        // or using `fugit` duration
        delay.delay(2.secs());
    }
}
