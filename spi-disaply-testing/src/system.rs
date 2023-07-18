use iron_coder_feather_rp2040_bsp;
use iron_coder_pitft_32_cap_touch_bsp;

use embedded_graphics::pixelcolor::Rgb565;
//use embedded_graphics::prelude::*;
use embedded_graphics_core::draw_target::DrawTarget;

pub struct System {
	pub main_board: iron_coder_feather_rp2040_bsp::Board,
	pub display: Option<iron_coder_pitft_32_cap_touch_bsp::Display<iron_coder_feather_rp2040_bsp::SPIBus, iron_coder_feather_rp2040_bsp::Cs, iron_coder_feather_rp2040_bsp::Dc, iron_coder_feather_rp2040_bsp::Rst>>,
}
impl System {
    pub fn new() -> Self {
        Self {
			main_board: iron_coder_feather_rp2040_bsp::Board::new(),
			display: None,
		}
    }

	pub fn init_display(&mut self) {
		let mut delay = self.main_board.delay_timer.take().unwrap();
		let spi = self.main_board.spi_bus.take().unwrap();
		let cs = self.main_board.d4.take().unwrap();
		let dc = self.main_board.d24.take().unwrap();
		let rst = self.main_board.d25.take().unwrap();
		self.display = Some(iron_coder_pitft_32_cap_touch_bsp::Display::init(spi, cs, dc, rst, delay));
	}

	pub fn clear_display(&mut self) {
		self.display.unwrap().0.clear(Rgb565::new(0, 0, 0));
	}

}
