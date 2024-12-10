use ratatui::style::Color;

pub const MIN_GRID_SIZE: usize = 3;
pub const MAX_GRID_SIZE: usize = 8; // exclusive
pub const GRID_SIZE_RANGE: std::ops::Range<usize> = MIN_GRID_SIZE..MAX_GRID_SIZE;
pub const SIZE_DEPTHS: [usize; MAX_GRID_SIZE] = [0, 0, 0, 6, 6, 5, 5, 4];

pub const BGCOLOR: Color = Color::Rgb(26, 26, 26);

pub const PREGAME_GRID_SIZE: f64 = 0.8;

pub const EXIT_CONFIRM_TEXT: &str = "Are you sure you want to exit?";
