use bevy::prelude::*;

mod game;
mod net;
mod menus;
use game::GamePlugin;
use menus::MainMenuPlugin;
use net::NetPlugin;

use crate::game::*;

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState {
    #[default]
    MainMenu,
    Hosting,
    Joining,
    Game,
    GameOver,
    Credits,
}

fn main() {
    App::new()
        .add_state::<AppState>()
        .add_plugins((
            GamePlugin,
            MainMenuPlugin,
            NetPlugin,
        ))
        .run();
}


