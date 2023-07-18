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
//use embedded_graphics_core::draw_target::DrawTarget;

#[entry]
fn main() -> ! {
    
    let mut system = System::new();

    // init code goes here!
	system.main_board.set_led(true);

	system.init_display();

	// Create a new character style
    let style = MonoTextStyle::new(&FONT_6X10, Rgb565::RED);

    // Create a text at position (20, 30) and draw it using the previously defined style
    Text::with_alignment(
        "First line\nSecond line",
        Point::new(20, 30),
        style,
        Alignment::Center,
    );
    //.draw(&mut system.display.unwrap().0)
    //.unwrap();

	//system.display.unwrap().0.clear(Rgb565::new(255, 0, 0));

    loop {
        // loop code goes here!


    }

}