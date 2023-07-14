#![no_std]
#![no_main]

/**** low-level imports *****/
use panic_halt as _;
use cortex_m_rt::entry;

use embedded_graphics::{
    mono_font::{ascii::FONT_6X10, MonoTextStyle},
    pixelcolor::BinaryColor,
    prelude::*,
    text::Text,
};

mod sys_mod_output_testing;
use sys_mod_output_testing::System;

#[entry]
fn main() -> ! {
    
    let mut system = System::new();
	let i2c_bus = system.feather_rp2040.i2c_bus.take().unwrap();
	system.oled_featherwing_128x64.init_display(i2c_bus);
	let mut display = system.oled_featherwing_128x64.display.take().unwrap();

	// Create a new character style
	let style = MonoTextStyle::new(&FONT_6X10, BinaryColor::On);

	let mut text = Text::new("Hello Rust!", Point::new(20, 30), style);

	// Create a text at position (20, 30) and draw it using the previously defined style
	text.draw(&mut display).unwrap();
	display.flush().unwrap();

    loop {
		system.feather_rp2040.delay(400);
        text.position.x += 1;
		text.draw(&mut display).unwrap();
		display.flush().unwrap();
		display.clear();
		system.feather_rp2040.set_led(text.position.x % 2 == 0);
    }

}