#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Io, Level, Output},
    prelude::*,
    timer::timg::TimerGroup,
};
use esp_println::{logger::init_logger_from_env, println};

#[embassy_executor::task]
async fn test_task() {
    log::info!("Test Task Starting");
    loop {
        log::info!("Test Task Running");
        embassy_time::Timer::after(embassy_time::Duration::from_millis(2_000)).await;
    }
}

#[main]
async fn main(spawner: embassy_executor::Spawner) {
    //Fix log output in vscode
    println!("\x1b[20h");

    let peripherals = esp_hal::peripherals::Peripherals::take();
    //let peripherals = esp_hal::init(esp_hal::Config::default()); //doesn't work ;<

    // Setup Time Driver for running async sleeps
    let system = esp_hal::system::SystemControl::new(peripherals.SYSTEM);
    let clocks = esp_hal::clock::ClockControl::max(system.clock_control).freeze();

    let timg0 = TimerGroup::new(peripherals.TIMG0, &clocks);
    esp_hal_embassy::init(&clocks, timg0.timer0);

    // Set GPIO0 as an output, and set its state high initially.
    let io = Io::new(peripherals.GPIO, peripherals.IO_MUX);
    let mut led = Output::new(io.pins.gpio11, Level::High);

    init_logger_from_env();

    log::info!("Main Starting");

    spawner.spawn(test_task()).ok();

    //Allow tasks to start before main loop is started
    embassy_futures::yield_now().await;

    loop {
        log::info!("Main Running");
        led.toggle();
        embassy_time::Timer::after(embassy_time::Duration::from_millis(1_000)).await;
        led.toggle();
        embassy_time::Timer::after(embassy_time::Duration::from_millis(1_000)).await;
    }

    //log::info!("Finished");
}
