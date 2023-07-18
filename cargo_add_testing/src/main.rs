#![no_std]
#![no_main]

/**** low-level imports *****/
use panic_halt as _;
//use cortex_m::entry;

mod system;
use system::System;

#[cortex_m_rt::entry]
fn main() -> ! {
    
    let mut system = System::new();

    // init code goes here!

    loop {
        // loop code goes here!
    }

}