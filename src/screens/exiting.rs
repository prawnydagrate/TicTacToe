use crate::{consts, helpers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Widget},
};

pub fn instructions() -> Vec<Span<'static>> {
    vec![
        " ←→".bold().blue(),
        " Change option".into(),
        "  ⏎".bold().blue(),
        " Select option ".into(),
    ]
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum ExitingState {
    #[default]
    Stay,
    Leave,
    Left,
}

pub struct ExitingWidget(pub helpers::Rfc<ExitingState>);

impl Widget for &ExitingWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = *self.0.borrow();
        let opts = if state == ExitingState::Stay {
            vec![" Stay ".bold().light_green(), " Leave ".dim()]
        } else {
            vec![" Stay ".dim(), " Leave ".bold().light_red()]
        };
        let options = Line::from(opts);
        Clear.render(area, buf);
        Block::bordered()
            .title(Line::from(" Exit? ".bold()).centered())
            .title_bottom(options.centered())
            .bg(consts::BGCOLOR)
            .border_set(border::ROUNDED)
            .render(area, buf);
        let wraplns = textwrap::wrap(
            consts::EXIT_CONFIRM_TEXT,
            (area.width as f64 * 0.7).round() as usize,
        );
        let height = wraplns.len() as u16;
        let mut width = 0;
        let text: String = wraplns
            .into_iter()
            .inspect(|ln| {
                width = std::cmp::max(width, ln.len() as u16);
            })
            .collect();
        Paragraph::new(text).centered().render(
            helpers::center(area, Constraint::Length(width), Constraint::Length(height)),
            buf,
        );
    }
}
