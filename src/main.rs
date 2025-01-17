#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::{
    gpio::{Level, Output},
    interrupt::{software::SoftwareInterruptControl, Priority},
    prelude::*,
    timer::{timg::TimerGroup, AnyTimer},
};
use esp_hal_embassy::InterruptExecutor;
use esp_println::println;
use static_cell::StaticCell;

/// Periodically print something.
#[embassy_executor::task]
async fn high_prio(mut led: Output<'static>) {
    println!("Starting high_prio()");
    loop {
        println!("High priority ticks");
        led.toggle();
        embassy_time::Timer::after(embassy_time::Duration::from_millis(500)).await;
        led.toggle();
        embassy_time::Timer::after(embassy_time::Duration::from_millis(1_000)).await;
    }
}

/// Simulates some blocking (badly behaving) task.
#[embassy_executor::task]
async fn low_prio_blocking() {
    println!("Starting low-priority task that isn't actually async");
    loop {
        println!("Doing some long and complicated calculation");
        let start = embassy_time::Instant::now();
        while start.elapsed() < embassy_time::Duration::from_secs(5) {}
        println!("Calculation finished");
        embassy_time::Timer::after(embassy_time::Duration::from_secs(5)).await;
    }
}

/// A well-behaved, but starved async task.
#[embassy_executor::task]
async fn low_prio_async() {
    println!("Starting low-priority task that will not be able to run while the blocking task is running");
    let mut ticker = embassy_time::Ticker::every(embassy_time::Duration::from_secs(1));
    loop {
        println!("Low priority ticks");
        ticker.next().await;
    }
}

#[main]
async fn main(low_priority_spawner: embassy_executor::Spawner) {
    esp_println::logger::init_logger_from_env();

    //Fix log output in vscode
    println!("\x1b[20h");

    log::info!("Main - Loading");

    let peripherals = esp_hal::init(esp_hal::Config::default()); //doesn't work ;<

    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let timer0: AnyTimer = timg0.timer0.into();
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let timer1: AnyTimer = timg1.timer0.into();

    esp_hal_embassy::init([timer0, timer1]);

    let led = Output::new(peripherals.GPIO11, Level::High);

    // // Setup Time Driver for running async sleeps
    // let system = esp_hal::system::SystemControl::new(peripherals.SYSTEM);
    // let clocks = esp_hal::clock::ClockControl::max(system.clock_control).freeze();
    // let timer0 = timg0.timer0;

    // Configure and Initialize LEDC Timer Driver
    // let mut ledc = Ledc::new(peripherals.LEDC, &clocks);
    // ledc.set_global_slow_clock(esp_hal::ledc::LSGlobalClkSource::APBClk);
    // let mut lstimer0 = ledc.get_timer::<esp_hal::ledc::LowSpeed>(esp_hal::ledc::timer::Number::Timer0);
    // lstimer0
    //     .configure(esp_hal::ledc::timer::config::Config {
    //         duty: esp_hal::ledc::timer::config::Duty::Duty5Bit,
    //         clock_source: esp_hal::ledc::timer::LSClockSource::APBClk,
    //         frequency: 24.kHz(),
    //     })
    //     .unwrap();

    static EXECUTOR: StaticCell<InterruptExecutor<2>> = StaticCell::new();
    let interrupt_executor = InterruptExecutor::new(sw_ints.software_interrupt2);
    let executor = EXECUTOR.init(interrupt_executor);

    let high_priority_spawner = executor.start(Priority::Priority3);

    log::info!("Main - Starting");

    high_priority_spawner.must_spawn(high_prio(led));

    println!("Spawning low-priority tasks");
    low_priority_spawner.must_spawn(low_prio_async());
    low_priority_spawner.must_spawn(low_prio_blocking());

    //log::info!("Finished");
}
