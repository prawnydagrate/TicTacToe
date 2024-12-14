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
        " ←→".bold().fg(consts::INSTRUCTIONS_COLOR),
        " Change option".into(),
        "  ⏎".bold().fg(consts::INSTRUCTIONS_COLOR),
        " Select option ".into(),
    ]
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum StartoverState {
    #[default]
    Stay,
    StartOver,
}
pub struct StartoverWidget(pub helpers::Rfc<StartoverState>);

impl Widget for &StartoverWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.0.borrow();
        let opts = if *state == StartoverState::Stay {
            vec![" Stay ".bold().light_green(), " Start over ".dim()]
        } else {
            vec![" Stay ".dim(), " Start over ".bold().light_red()]
        };
        let options = Line::from(opts);
        Clear.render(area, buf);
        Block::bordered()
            .title(Line::from("Start over?".bold()).centered())
            .title_bottom(options.centered())
            .bg(consts::BGCOLOR)
            .border_set(border::ROUNDED)
            .render(area, buf);
        let wraplns = textwrap::wrap(
            consts::STARTOVER_TEXT,
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
