#![no_std]
pub struct System {
    feather_rp2040: todo_get_bsp_name,
    propmaker_featherwing: todo_get_bsp_name,
    num_boards: u8,
}
impl System {
    pub fn new() -> Self {
        Self { num_boards: 2usize }
    }
}
