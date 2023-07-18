use iron_coder_pitft_32_cap_touch_bsp;
use iron_coder_feather_rp2040_bsp;
pub struct System {
	main_board: iron_coder_feather_rp2040_bsp::Board,
	display: iron_coder_pitft_32_cap_touch_bsp::Display<iron_coder_feather_rp2040_bsp::SPIBus>,
}
impl System {
    
}
