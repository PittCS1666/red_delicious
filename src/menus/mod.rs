use bevy::prelude::*;
mod layout;
mod interactions;
mod components;

use crate::AppState;
use layout::*;
use interactions::*;

#[derive(Resource)]
pub struct NetworkAddresses {
    pub host_port: String, //host port
    pub client_port: String,
    pub ip: String,
}

pub struct MainMenuPlugin;

impl Plugin for MainMenuPlugin{
    fn build(&self, app: &mut App) {
        app
        .add_systems(OnEnter(AppState::MainMenu), spawn_main_menu)
        .add_systems(OnExit(AppState::MainMenu), despawn_main_menu)
        .add_systems(OnEnter(AppState::Credits), spawn_credits_page)
        .add_systems(Update, show_popup)
        .add_systems(OnExit(AppState::Credits), despawn_credits_page)
        .add_systems(OnEnter(AppState::Hosting), spawn_host_page)
        .add_systems(OnExit(AppState::Hosting), despawn_host_page)
        .add_systems(OnEnter(AppState::Joining), spawn_join_page)
        .add_systems(OnExit(AppState::Joining), despawn_join_page)
        .add_systems(OnEnter(AppState::Controls), spawn_controls_page)
        .add_systems(OnExit(AppState::Controls), despawn_controls_page)
        .add_systems(OnEnter(AppState::Game), spawn_in_game_menu)
        .add_systems(OnExit(AppState::Game), despawn_in_game_menu)
        .add_systems(OnEnter(AppState::Game), spawn_game_over_screen)
        .add_systems(OnExit(AppState::Game), despawn_game_over_screen)
        .add_systems(Update, interact_with_host_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_join_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_controls_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_credits_button.run_if(in_state(AppState::MainMenu)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Hosting)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Joining)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Controls)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Credits)))
        .add_systems(Update, interact_with_back_button.run_if(in_state(AppState::Game)))
        .add_systems(Update, interact_with_credits_button.run_if(in_state(AppState::Game)))
        .add_systems(Update, update_host_input)
        .add_systems(Update, update_time_remaining_system)
        .add_systems(Update, save_host_input)
        .add_systems(Update, update_join_port_input)
        .add_systems(Update, update_join_host_port_input)
        .add_systems(Update, join_host_port_but)
        .add_systems(Update, update_join_ip_input)
        .add_systems(Update, join_port_but)
        .add_systems(Update, join_ip_but)
        .add_systems(Update, save_join_input)
        .add_systems(Update, init_host_port_input_system)
        .add_systems(Update, init_join_host_port_input_system)
        .add_systems(Update, init_join_port_input_system)
        .add_systems(Update, init_join_ip_input_system)
        .add_systems(Startup, startup);
}}

pub fn startup(mut commands: Commands) {
    commands.insert_resource( NetworkAddresses {
        host_port: String::new(), client_port: String::new(), ip: String::new(),
    });
}