#![no_std]
#![no_main]

use embedded_graphics::{
    mono_font::{
        ascii::FONT_9X18_BOLD,
        MonoTextStyleBuilder,
    },
    pixelcolor::BinaryColor,
    prelude::*,
    text::{Alignment, Text},
};
use hal::{
    clock::ClockControl,
    gpio::IO,
    i2c::I2C,
    peripherals::Peripherals,
    prelude::*,
    timer::TimerGroup,
};
use esp_backtrace as _;
use nb::block;
use ssd1306::{prelude::*, I2CDisplayInterface, Ssd1306};
use micromath::F32Ext;

#[entry]
fn main() -> ! {
    // Setup timers
    let peripherals = Peripherals::take();
    let mut system = peripherals.SYSTEM.split();
    let clocks = ClockControl::boot_defaults(system.clock_control).freeze();

    let timer_group0 = TimerGroup::new(
        peripherals.TIMG0,
        &clocks,
        &mut system.peripheral_clock_control,
    );
    let mut timer0 = timer_group0.timer0;

    // Setup IO
    let io = IO::new(peripherals.GPIO, peripherals.IO_MUX);

    // Setup I2C with
    // - SDA => GPIO1
    // - SCL => GPIO2
    let i2c = I2C::new(
        peripherals.I2C0,
        io.pins.gpio1,
        io.pins.gpio2,
        100u32.kHz(),
        &mut system.peripheral_clock_control,
        &clocks,
    );

    // Start timer (5 second interval)
    timer0.start(5u64.secs());

    // Initialize display with I2C
    let interface = I2CDisplayInterface::new(i2c);
    let mut display = Ssd1306::new(interface, DisplaySize128x64, DisplayRotation::Rotate0)
        .into_buffered_graphics_mode();
    display.init().unwrap();

    // Specify different text styles
    let text_style = MonoTextStyleBuilder::new()
        .font(&FONT_9X18_BOLD)
        .text_color(BinaryColor::On)
        .build();

    loop {
        // Write crustyahh
        Text::with_alignment(
            "crustyahh",
            display.bounding_box().center() + Point::new(0, 0),
            text_style,
            Alignment::Center,
        )
        .draw(&mut display)
        .unwrap();

        // Write buffer to display
        display.flush().unwrap();
        // Clear display buffer
        display.clear(BinaryColor::Off).unwrap();

        // Wait on timer
        block!(timer0.wait()).unwrap();
    }
}
