use iron_coder_feather_rp2040_bsp;
use iron_coder_featherwing_propmaker_bsp;
pub struct System {
    pub feather_rp2040: iron_coder_feather_rp2040_bsp::Board,
}
impl System {
    pub fn new() -> Self {
        Self {
            feather_rp2040: iron_coder_feather_rp2040_bsp::Board::new(),
        }
    }
}
