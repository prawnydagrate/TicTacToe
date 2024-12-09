use crate::helpers::{centered_scale, Rfc};
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

pub struct ExitingWidget(pub Rfc<ExitingState>);

impl Widget for &ExitingWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let area = centered_scale(area, 0.6, 0.3);
        let state = *self.0.borrow();
        let opts = if state == ExitingState::Stay {
            vec![" Stay ".bold().light_green(), " Leave ".dim()]
        } else {
            vec![" Stay ".dim(), " Leave ".bold().light_red()]
        };
        let options = Line::from(opts);
        let block = Block::bordered()
            .title(Line::from(" Exit? ".bold()).centered())
            .title_bottom(options.centered())
            .border_set(border::ROUNDED);
        Paragraph::new("Are you sure you want to exit?")
            .centered()
            .block(block)
            .render(area, buf);
    }
}
