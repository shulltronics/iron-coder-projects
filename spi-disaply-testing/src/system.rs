use iron_coder_feather_rp2040_bsp;
use iron_coder_pitft_32_cap_touch_bsp;

use embedded_graphics::pixelcolor::Rgb565;
//use embedded_graphics::prelude::*;
use embedded_graphics::draw_target::DrawTarget;

type I2CType<'a> = shared_bus::I2cProxy<'a, shared_bus::NullMutex<iron_coder_feather_rp2040_bsp::I2CBus>>;

pub struct System<'a> {
	pub main_board: iron_coder_feather_rp2040_bsp::Board,
	pub display: iron_coder_pitft_32_cap_touch_bsp::Display<iron_coder_feather_rp2040_bsp::SPIBus, iron_coder_feather_rp2040_bsp::Dc, iron_coder_feather_rp2040_bsp::Cs, iron_coder_feather_rp2040_bsp::Rst, I2CType<'a>>,
}
impl<'a> System<'a> {
    pub fn new() -> Self {
        Self {
			main_board: iron_coder_feather_rp2040_bsp::Board::new(),
			display:  iron_coder_pitft_32_cap_touch_bsp::Display::new(),
		}
    }

	pub fn init_display(&mut self) {
		//let mut delay = self.main_board.delay_timer.take().unwrap();
		let spi = self.main_board.spi_bus.take().unwrap();
		let cs = self.main_board.d4.take().unwrap();
		let dc = self.main_board.d25.take().unwrap();
		let rst = self.main_board.d5.take().unwrap();
		self.display.init_display(spi, dc, cs, rst, &mut self.main_board.delay_timer);
	}

	pub fn init_touchscreen(&'a mut self) {
		let i2c = self.main_board.i2c_manager.acquire_i2c();
		self.display.init_touchscreen(i2c);
	}

	pub fn clear_display(&mut self) {
		self.display.display.as_mut().unwrap().clear(Rgb565::new(0, 0, 0));
	}

}
