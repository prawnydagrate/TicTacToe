use crate::{consts, helpers};
use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Rect},
    style::Stylize,
    symbols::border,
    text::{Line, Span},
    widgets::{Block, Clear, Paragraph, Widget},
};
use super::pregame;

pub fn instructions() -> Vec<Span<'static>> {
    vec![
        " ←→".bold().blue(),
        " Change option".into(),
        "  ⏎".bold().blue(),
        " Select option ".into(),
    ]
}

#[derive(Debug, Eq, PartialEq, Copy, Clone, Default)]
pub enum PregameConfirmOptionState {
    #[default]
    X,
    O
}

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub struct PregameConfirmState {
    pub pregame_state: pregame::PregameState,
    pub option_state: PregameConfirmOptionState,
}

pub struct PregameConfirmWidget(pub helpers::Rfc<PregameConfirmState>);

impl Widget for &PregameConfirmWidget {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let state = self.0.borrow();
        let opts = if state.option_state == PregameConfirmOptionState::X {
            vec![" X ".bold().light_blue(), " O ".dim()]
        } else {
            vec![" X ".dim(), " O ".bold().light_blue()]
        };
        let options = Line::from(opts);
        Clear.render(area, buf);
        let gsize = state.pregame_state.grid_size;
        Block::bordered()
            .title(Line::from(format!(" {gsize}x{gsize} Tic Tac Toe ").bold()).centered())
            .title_bottom(options.centered())
            .bg(consts::BGCOLOR)
            .border_set(border::ROUNDED)
            .render(area, buf);
        let wraplns = textwrap::wrap(
            consts::PREGAME_CONFIRM_TEXT,
            (area.width as f64 * 0.7).round() as usize,
        );
        let height = wraplns.len() as u16;
        let mut width = 0;
        let text: String = wraplns
            .into_iter()
            .map(|ln| {
                width = std::cmp::max(width, ln.len() as u16);
                ln
            })
            .collect();
        Paragraph::new(text).centered().render(
            helpers::center(area, Constraint::Length(width), Constraint::Length(height)),
            buf,
        );
    }
}
