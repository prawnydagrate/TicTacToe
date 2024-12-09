use crate::{
    helpers,
    screens::{exiting, ingame, pregame},
};

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct AppState<'a> {
    pub pregame: Option<helpers::Rfc<pregame::PregameState>>,
    pub ingame: Option<helpers::Rfc<ingame::IngameState<'a>>>,
    pub exiting: Option<helpers::Rfc<exiting::ExitingState>>,
}

impl Default for AppState<'_> {
    fn default() -> Self {
        Self {
            pregame: Some(helpers::rfc(pregame::PregameState::default())),
            ingame: None,
            exiting: None,
        }
    }
}
