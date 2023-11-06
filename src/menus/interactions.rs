use bevy::prelude::*;

use crate::menus::components::*;
use crate::AppState;
use crate::game::PlayerId;
use crate::menus::NetworkAddresses;

pub fn interact_with_button<B: ButtonTypeTrait>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<B::Marker>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(B::app_state());
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color::rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
pub fn interact_with_host_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<HostButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<HostButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_join_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<JoinButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<JoinButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_back_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<BackToMainMenu>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<BackButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_controls_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<ControlsButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<ControlsButtonType>(button_query, app_state_next_state);
}

pub fn interact_with_credits_button(
    button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<CreditsButton>)>,
    app_state_next_state: ResMut<NextState<AppState>>,
) {
    interact_with_button::<CreditsButtonType>(button_query, app_state_next_state);
}

pub fn update_input<T: InputType>(
    mut char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Text, &mut T)>,
    switch_query: Option<Query<&Switch>>,
) {
    if let Some(mut switch_query) = switch_query {
        let mut active = false;
        for switch in switch_query.iter_mut() {
            active = T::is_active(switch);
        }
        if !T::is_valid(active) {
            return;
        }
    }
    let mut new_char = None;
    for event in char_events.iter() {
        new_char = Some(event.char);
    }

    if let Some(new_char) = new_char {
        for (mut text, mut input_type) in query.iter_mut() {
            if new_char != '\u{8}' && new_char != '\u{7f}' {
                text.sections[0].value.push(new_char);
                input_type.push_char(new_char);
            }
            else{
                if !input_type.is_empty() {
                    text.sections[0].value.pop();
                    input_type.pop_char();
                }
            }
        }
    }
}
pub fn update_host_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut HostPortInput)>,
) {
    update_input::<HostPortInput>(char_events, keyboard_input, query, None);
}

pub fn update_join_port_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut JoinPortInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinPortInput>(char_events, keyboard_input, query, Some(switch_query));
}

pub fn update_join_host_port_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut JoinHostPortInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinHostPortInput>(char_events, keyboard_input, query, Some(switch_query));
}

pub fn update_join_ip_input(
    char_events: EventReader<ReceivedCharacter>,
    keyboard_input: Res<Input<KeyCode>>,
    query: Query<(&mut Text, &mut JoinIPInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinIPInput>(char_events, keyboard_input, query, Some(switch_query));
}

pub fn save_host_input(
    mut is_host: ResMut<crate::net::IsHost>,
    mut res_id: ResMut<PlayerId>,
    mut net_address: ResMut<NetworkAddresses>,
    host_port_query: Query<&HostPortInput>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HostPortSaveBut>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for host_port_input in host_port_query.iter() {
                    net_address.host_port = host_port_input.port.clone();
                    res_id.0 = 0;
                    println!("setting res_id to {:?}", res_id.0);
                    is_host.0 = true;
                }
                app_state_next_state.set(AppState::Respawn);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
pub fn join_port_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinPortBut>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host_port = false;
                    switch.ip = false;
                    switch.port = true;
                }
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
pub fn join_ip_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinIpBut>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host_port = false;
                    switch.ip = true;
                    switch.port = false;
                }
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
pub fn join_host_port_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinHostPortBut>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host_port = true;
                    switch.ip = false;
                    switch.port = false;
                }
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}
pub fn save_join_input(
    mut is_host: ResMut<crate::net::IsHost>,
    mut net_address: ResMut<NetworkAddresses>,
    join_port_query: Query<&JoinPortInput>,
    join_ip_query: Query<&JoinIPInput>,
    join_host_port_query: Query<&JoinHostPortInput>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinSaveBut>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for join_port_input in join_port_query.iter() {
                    net_address.client_port = join_port_input.port.clone();
                }
                for join_ip_input in join_ip_query.iter() {
                    net_address.ip = join_ip_input.ip.clone();
                }
                for join_host_port_input in join_host_port_query.iter() {
                    net_address.host_port =join_host_port_input.port.clone();
                }
                is_host.0 = false;
                app_state_next_state.set(AppState::Respawn);
            }
            Interaction::Hovered => {
                *background_color = Color::GRAY.into();
            }
            Interaction::None => {
                *background_color = Color:: rgb(0.15, 0.15, 0.15).into();
            }
        }
    }
}

pub fn init_input_system_with_default<T: InputType>(
    default_value: &str,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Text, &mut T), Without<Initialized>>,
) {
    for (entity, mut text, mut input) in query.iter_mut() {
        if input.is_empty() {
            text.sections[0].value.push_str(&format!(" {}", default_value));
            for ch in default_value.chars() {
                input.push_char(ch);
            }
            commands.entity(entity).insert(Initialized {});
        }
    }
}
//adjust the default of each inputs here
pub fn init_host_port_input_system(
    commands: Commands,
    host_port_query: Query<(Entity, &mut Text, &mut HostPortInput), Without<Initialized>>,
) {
    init_input_system_with_default::<HostPortInput>("8085", commands, host_port_query);
}

pub fn init_join_host_port_input_system(
    commands: Commands,
    join_host_port_query: Query<(Entity, &mut Text, &mut JoinHostPortInput), Without<Initialized>>,
) {
    init_input_system_with_default::<JoinHostPortInput>("8085", commands, join_host_port_query);
}

pub fn init_join_port_input_system(
    commands: Commands,
    join_port_query: Query<(Entity, &mut Text, &mut JoinPortInput), Without<Initialized>>,
) {
    init_input_system_with_default::<JoinPortInput>("8086", commands, join_port_query);
}

pub fn init_join_ip_input_system(
    commands: Commands,
    join_ip_query: Query<(Entity, &mut Text, &mut JoinIPInput), Without<Initialized>>,
) {
    init_input_system_with_default::<JoinIPInput>("127.0.0.1", commands, join_ip_query);
}




