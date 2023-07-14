use iron_coder_feather_rp2040_bsp;
use iron_coder_featherwing_oled_bsp;
pub struct System {
    pub feather_rp2040: iron_coder_feather_rp2040_bsp::Board,
    pub oled_featherwing_128x64: iron_coder_featherwing_oled_bsp::Board<
        iron_coder_feather_rp2040_bsp::I2CBus,
    >,
}
impl System {
    pub fn new() -> Self {
        Self {
            feather_rp2040: iron_coder_feather_rp2040_bsp::Board::new(),
            oled_featherwing_128x64: iron_coder_featherwing_oled_bsp::Board::new(),
        }
    }
}
