use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use toetactic_lib::mech::{Game, Move};

use crate::helpers::Rfc;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IngameState {
    game: Game,
    selected: Move,
}

pub struct IngameWidget(pub Rfc<IngameState>);

impl Widget for &IngameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
