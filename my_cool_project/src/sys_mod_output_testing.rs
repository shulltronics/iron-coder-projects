use iron_coder_feather_rp2040_bsp;
use iron_coder_featherwing_propmaker_bsp;
pub struct System {
    feather_rp2040: iron_coder_feather_rp2040_bsp::Board,
    propmaker_featherwing: Board<iron_coder_feather_rp2040_bsp::I2CBus>,
}
impl System {
    pub fn new() -> Self {
        Self {
            feather_rp2040: iron_coder_feather_rp2040_bsp::Board::new(),
            propmaker_featherwing: iron_coder_featherwing_propmaker_bsp::Board::new(),
        }
    }
}
