use crate::consts;

#[derive(Debug, Eq, PartialEq, Copy, Clone)]
pub struct PregameState {
    grid_size: usize,
}

impl Default for PregameState {
    fn default() -> Self {
        Self {
            grid_size: consts::MIN_GRID_SIZE,
        }
    }
}
