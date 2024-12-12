pub mod exiting;
pub mod ingame;
pub mod pregame;
pub mod pregame_confirm;

#[derive(Debug, Eq, PartialEq, Clone, Default)]
pub enum CurrentScreen {
    #[default]
    Pregame,
    PregameConfirm,
    Ingame,
    Exiting(Box<CurrentScreen>),
}
