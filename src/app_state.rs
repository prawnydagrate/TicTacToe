use crate::{
    helpers,
    screens::{exiting, ingame, pregame},
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AppState {
    pub pregame: Option<helpers::Rfc<pregame::PregameState>>,
    pub ingame: Option<helpers::Rfc<ingame::IngameState>>,
    pub exiting: Option<helpers::Rfc<exiting::ExitingState>>,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            pregame: Some(helpers::rfc(pregame::PregameState::default())),
            ingame: None,
            exiting: None,
        }
    }
}
