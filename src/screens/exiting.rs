use crate::{consts, helpers::{self, centered_scale, Rfc}};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::{Color, Stylize},
    symbols::border,
    text::{Line, Text},
    widgets::{Block, Clear, Paragraph, Widget},
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
            .bg(consts::BGCOLOR)
            .border_set(border::ROUNDED);
        Paragraph::new(consts::EXIT_CONFIRM_TEXT)
            .centered()
            .block(block)
            .render(area, buf);
    }
}
