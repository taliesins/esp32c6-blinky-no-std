#![no_std]
#![no_main]
#![feature(type_alias_impl_trait)]

use esp_backtrace as _;
use esp_hal::{
    gpio::Level,
    interrupt::{software::SoftwareInterruptControl, Priority},
    prelude::*,
    timer::{timg::TimerGroup, AnyTimer},
};
use esp_hal_embassy::InterruptExecutor;
use esp_println::println;
use smart_leds::SmartLedsWrite;
use static_cell::StaticCell;

/// Periodically print something.
#[embassy_executor::task]
async fn blink_led_task(mut led: esp_hal::gpio::Output<'static>) {
    println!("High priority - blink led starting");
    loop {
        led.set_high();
        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;

        led.set_low();
        embassy_time::Timer::after(embassy_time::Duration::from_secs(1)).await;
    }
}

#[embassy_executor::task]
async fn blink_rgb_led_task(
    mut rgb_led_adapter: esp_hal_smartled::SmartLedsAdapter<
        esp_hal::rmt::Channel<esp_hal::Blocking, 0>,
        25,
    >,
) {
    println!("Low priority - blink rgb led starting");

    // We use one of the RMT channels to instantiate a `SmartLedsAdapter` which can
    // be used directly with all `smart_led` implementations

    let mut color = smart_leds::hsv::Hsv {
        hue: 0,
        sat: 255,
        val: 255,
    };
    let mut data;

    let mut ticker = embassy_time::Ticker::every(embassy_time::Duration::from_millis(20));
    loop {
        // Iterate over the rainbow!
        for hue in 0..=255 {
            color.hue = hue;
            // Convert from the HSV color space (where we can easily transition from one
            // color to the other) to the RGB color space that we can then send to the LED
            data = [smart_leds::hsv::hsv2rgb(color)];
            // When sending to the LED, we do a gamma correction first (see smart_leds
            // documentation for details) and then limit the brightness to 10 out of 255 so
            // that the output it's not too bright.

            esp_hal_smartled::SmartLedsAdapter::write(
                &mut rgb_led_adapter,
                smart_leds::brightness(smart_leds::gamma(data.iter().cloned()), 50),
            )
            .unwrap();
            ticker.next().await;
        }
    }
}

#[main]
async fn main(low_priority_spawner: embassy_executor::Spawner) {
    esp_println::logger::init_logger_from_env();

    //Fix log output in vscode
    println!("\x1b[20h");

    log::info!("Main - Starting");

    let peripherals = esp_hal::init(esp_hal::Config::default()); //doesn't work ;<

    let sw_ints = SoftwareInterruptControl::new(peripherals.SW_INTERRUPT);

    let timg0 = esp_hal::timer::timg::TimerGroup::new(peripherals.TIMG0);
    let timer0: AnyTimer = timg0.timer0.into();
    let timg1 = TimerGroup::new(peripherals.TIMG1);
    let timer1: AnyTimer = timg1.timer0.into();

    esp_hal_embassy::init([timer0, timer1]);

    // Onboard RGB LED
    let freq = 80.MHz();
    let rmt = esp_hal::rmt::Rmt::new(peripherals.RMT, freq).unwrap();
    let rmt_buffer: [u32; 25] = esp_hal_smartled::smartLedBuffer!(1);
    let rgb_led_adapter =
        esp_hal_smartled::SmartLedsAdapter::new(rmt.channel0, peripherals.GPIO8, rmt_buffer);

    // External LED
    let led_pin = esp_hal::gpio::Output::new(peripherals.GPIO4, Level::Low);

    static EXECUTOR: StaticCell<InterruptExecutor<2>> = StaticCell::new();
    let interrupt_executor = InterruptExecutor::new(sw_ints.software_interrupt2);
    let executor = EXECUTOR.init(interrupt_executor);

    let high_priority_spawner = executor.start(Priority::Priority3);

    log::info!("Spawning high-priority tasks");
    high_priority_spawner.must_spawn(blink_led_task(led_pin));

    println!("Spawning low-priority tasks");
    low_priority_spawner.must_spawn(blink_rgb_led_task(rgb_led_adapter));

    loop {
        log::info!("Main Loop");
        embassy_time::Timer::after(embassy_time::Duration::from_millis(1000)).await;
    }

    // log::info!("Finished");
}
