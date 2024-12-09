use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};

use crate::{consts, helpers::Rfc};

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

pub struct PregameWidget(pub Rfc<PregameState>);

impl Widget for &PregameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
