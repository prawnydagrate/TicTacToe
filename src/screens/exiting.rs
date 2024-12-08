use crate::{app_state::PassedState, helpers::centered_scale};
use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::Stylize,
    symbols::border,
    text::Line,
    widgets::{Block, Paragraph, Widget},
};

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum ExitingState {
    #[default]
    Stay,
    Leave,
    Left,
}

pub struct ExitingWidget(pub PassedState<ExitingState>);

impl Widget for &ExitingWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_scale(area, 0.6, 0.3);
        let block = Block::bordered()
            .title(Line::from(" Exit? ".bold()).centered())
            .border_set(border::ROUNDED);
        Paragraph::new("").centered().block(block).render(area, buf);
    }
}
