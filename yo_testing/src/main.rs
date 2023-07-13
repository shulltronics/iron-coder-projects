#![no_std]
#![no_main]

/**** low-level imports *****/
use panic_halt as _;
use cortex_m_rt::entry;

mod sys_mod_output_testing;
use sys_mod_output_testing::System;

#[entry]
fn main() -> ! {
    // Grab the singleton objects
    let mut system = System::new();

	let i2c_bus = system.feather_rp2040.i2c_bus.take().unwrap();

	system.propmaker_featherwing.init_accelerometer(i2c_bus);
    
    loop {
        system.feather_rp2040.set_led(true);
		system.feather_rp2040.delay(500);
		system.feather_rp2040.set_led(false);
		system.feather_rp2040.delay(500);
		let a = system.propmaker_featherwing.read_accelerometer();
		system.feather_rp2040.serial_write(a);
    }

}