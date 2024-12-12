use ratatui::{buffer::Buffer, layout::Rect, widgets::Widget};
use toetactic_lib::mech::{Game, Move, Player};
use crate::helpers;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct IngameState {
    pub game: Game,
    pub turn: Player,
    pub selected: Move,
}

pub struct IngameWidget(pub helpers::Rfc<IngameState>);

impl Widget for &IngameWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {}
}
