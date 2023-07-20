#![no_std]
#![no_main]

/**** low-level imports *****/
use panic_halt as _;
use cortex_m_rt::entry;

mod system;
use system::System;

use embedded_graphics::{
	prelude::*,
	text::{Alignment, Text},
	mono_font::{ascii::FONT_6X10, MonoTextStyle},
	pixelcolor::Rgb565,
};

use smart_leds::RGB8;

#[entry]
fn main() -> ! {
    
    let mut system = System::new();

    // init code goes here!
	system.main_board.set_led(true);

	system.init_display();
	//system.init_touchscreen();
	system.main_board.delay(1000);

	let i2c = system.main_board.i2c_manager.acquire_i2c();
	let mut ts = ft6236::FT6236::new(i2c);
	let n = match ts.get_number_of_touches() {
		Ok(n) => n,//system.main_board.serial_write(n),
		Err(e) => 100,//system.main_board.serial_write(e),
	};

	system.main_board.serial_write(n);
	//system.display.init_touchscreen(i2c);
	// system.clear_display();
    system.display.display.as_mut().unwrap().clear(Rgb565::new(0, 0, 0));

	// Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::with_alignment(
        "First line\nSecond line",
        Point::new(100, 30),
        style,
        Alignment::Center,
    )
    .draw(&mut system.display.display.unwrap())
    .unwrap();

	let colors: [RGB8; 3] = [
		RGB8::new(255, 0, 0),
		RGB8::new(0, 255, 0),
		RGB8::new(0, 0, 255),
	];

	let mut color_iter = colors.iter().cycle();
	//let mut ts = system.display.touchscreen.take().unwrap();

    loop {
		system.main_board.set_neopixel_color(*color_iter.next().unwrap());
        // loop code goes here!
		system.main_board.set_led(true);
		system.main_board.delay(500);
		system.main_board.set_led(false);
		system.main_board.delay(500);
		system.main_board.serial_write(n);
    }

}