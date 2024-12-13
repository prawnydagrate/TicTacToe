pub mod app;
mod app_state;
mod consts;
mod helpers;
mod screens;

pub use app::App;

pub type AppResult = std::io::Result<()>;
