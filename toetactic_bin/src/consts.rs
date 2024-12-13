use ratatui::style::Color;

pub const MIN_GRID_SIZE: usize = 3;
pub const MAX_GRID_SIZE: usize = 7;
pub const SIZE_DEPTHS: [usize; MAX_GRID_SIZE + 1] = [0, 0, 0, 6, 9, 6, 5, 5];

pub const BGCOLOR: Color = Color::Rgb(26, 26, 26);

pub const PREGAME_GRID_SIZE: f64 = 0.8;

pub const PREGAME_CONFIRM_TEXT: &str = "Would you like to play X or O? (X plays first)";

pub const INGAME_GRID_SIZE: f64 = 0.8;

pub const EXIT_CONFIRM_TEXT: &str = "Are you sure you want to exit?";
