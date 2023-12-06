use bevy::prelude::*;

use crate::menus::components::*;
use crate::AppState;
use crate::game::PlayerId;
use crate::menus::NetworkAddresses;
use crate::game::MapConfig;
use rand::Rng;
use bevy::app::AppExit;

pub fn interact_with_button<B: ButtonTypeTrait>(
    mut button_query: Query<(&Interaction, &mut BackgroundColor), (Changed<Interaction>, With<B::Marker>)>,
    mut app_state_next_state: ResMut<NextState<AppState>>,
    mut commands: Commands,
    asset_server: Res<AssetServer>
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                *background_color = Color::GRAY.into();
                app_state_next_state.set(B::app_state());
                commands.spawn(AudioBundle {
                    source: asset_server.load("click.ogg"),
                    ..default()
                });
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

pub fn update_input<T: InputType>(
    mut char_events: EventReader<ReceivedCharacter>,
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
    query: Query<(&mut Text, &mut HostPortInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<HostPortInput>(char_events, query, Some(switch_query));
}

pub fn update_num_camps_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut NumCampsInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<NumCampsInput>(char_events, query, Some(switch_query));
}

pub fn update_num_chests_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut NumChestsInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<NumChestsInput>(char_events, query, Some(switch_query));
}

pub fn update_enemies_per_camp_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut EnemiesPerCampInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<EnemiesPerCampInput>(char_events, query, Some(switch_query));
}

pub fn update_map_seed_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut MapSeedInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<MapSeedInput>(char_events, query, Some(switch_query));
}

pub fn update_eid_percentage_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut EidPercentageInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<EidPercentageInput>(char_events, query, Some(switch_query));
}

pub fn update_join_port_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut JoinPortInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinPortInput>(char_events, query, Some(switch_query));
}

pub fn update_join_host_port_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut JoinHostPortInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinHostPortInput>(char_events, query, Some(switch_query));
}

pub fn update_join_ip_input(
    char_events: EventReader<ReceivedCharacter>,
    query: Query<(&mut Text, &mut JoinIPInput)>,
    switch_query: Query<&Switch>,
) {
    update_input::<JoinIPInput>(char_events, query, Some(switch_query));
}

pub fn save_host_input(
    mut is_host: ResMut<crate::net::IsHost>,
    mut res_id: ResMut<PlayerId>,
    mut net_address: ResMut<NetworkAddresses>,
    mut map_config: ResMut<MapConfig>,
    host_port_query: Query<&HostPortInput>,
    num_camps_query: Query<&NumCampsInput>,
    num_chests_query: Query<&NumChestsInput>,
    enemy_per_camp_query: Query<&EnemiesPerCampInput>,
    map_seed_query: Query<&MapSeedInput>,
    eid_percentage_query: Query<&EidPercentageInput>,
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HostPortSaveButton>),
    >,
    mut app_state_next_state: ResMut<NextState<AppState>>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for host_port_input in host_port_query.iter() {
                    net_address.host_port = host_port_input.port.clone();
                    res_id.0 = 0;
                    is_host.0 = true;
                }
                for num_camps_input in  num_camps_query.iter() {
                    map_config.num_camps = num_camps_input.value.clone();
                }
                for input in  num_chests_query.iter() {
                    map_config.num_chests = input.value.clone();
                }
                for input in  enemy_per_camp_query.iter() {
                    map_config.enemy_per_camp = input.value.clone();
                }
                for input in  map_seed_query.iter() {
                    map_config.map_seed = input.value.clone();
                }
                for input in  eid_percentage_query.iter() {
                    map_config.eid_percentage = input.value.clone();
                    //println!("eid percentage to {:?}", map_config.eid_percentage);
                }
                app_state_next_state.set(AppState::Game);
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

pub fn host_port_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<HostPortButton>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host = true;
                    switch.num_camps = false;
                    switch.num_chests = false;
                    switch.enemy_per_camp = false;
                    switch.map_seed = false;
                    switch.eid_percentage = false;
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

pub fn num_camps_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NumCampsButton>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host = false;
                    switch.num_camps = true;
                    switch.num_chests = false;
                    switch.enemy_per_camp = false;
                    switch.map_seed = false;
                    switch.eid_percentage = false;
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
pub fn num_chests_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<NumChestsButton>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host = false;
                    switch.num_camps = false;
                    switch.num_chests = true;
                    switch.enemy_per_camp = false;
                    switch.map_seed = false;
                    switch.eid_percentage = false;
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

pub fn enemy_per_camp_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EnemiesPerCampButton>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host = false;
                    switch.num_camps = false;
                    switch.num_chests = false;
                    switch.enemy_per_camp = true;
                    switch.map_seed = false;
                    switch.eid_percentage = false;
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

pub fn map_seed_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<MapSeedButton>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host = false;
                    switch.num_camps = false;
                    switch.num_chests = false;
                    switch.enemy_per_camp = false;
                    switch.map_seed = true;
                    switch.eid_percentage = false;
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
pub fn eid_percentage_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<EidPercentageButton>),
    >,
    mut switch_query: Query<&mut Switch>,
) {
    if let Ok((interaction, mut background_color)) = button_query.get_single_mut() {
        match *interaction {
            Interaction::Pressed => {
                for mut switch in switch_query.iter_mut() {
                    switch.host = false;
                    switch.num_camps = false;
                    switch.num_chests = false;
                    switch.enemy_per_camp = false;
                    switch.map_seed = false;
                    switch.eid_percentage = true;
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

pub fn join_port_but(
    mut button_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<JoinPortButton>),
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
        (Changed<Interaction>, With<JoinIpButton>),
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
        (Changed<Interaction>, With<JoinHostPortButton>),
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
        (Changed<Interaction>, With<JoinSaveButton>),
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
                app_state_next_state.set(AppState::Connecting);
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
    init_input_system_with_default::<JoinIPInput>("10.4.43.37", commands, join_ip_query);
}

pub fn init_num_camps_input_system(
    commands: Commands,
    num_camps_query: Query<(Entity, &mut Text, &mut NumCampsInput), Without<Initialized>>,
) {
    init_input_system_with_default::<NumCampsInput>("10", commands, num_camps_query);
}

pub fn init_num_chests_input_system(
    commands: Commands,
    num_chests_query: Query<(Entity, &mut Text, &mut NumChestsInput), Without<Initialized>>,
) {
    init_input_system_with_default::<NumChestsInput>("WIP", commands, num_chests_query);
}

pub fn init_enemies_per_camp_input_system(
    commands: Commands,
    enemies_per_camp_query: Query<(Entity, &mut Text, &mut EnemiesPerCampInput), Without<Initialized>>,
) {
    init_input_system_with_default::<EnemiesPerCampInput>("WIP", commands, enemies_per_camp_query);
}

pub fn init_map_seed_input_system(
    commands: Commands,
    map_seed_query: Query<(Entity, &mut Text, &mut MapSeedInput), Without<Initialized>>,
) {
    let mut rng = rand::thread_rng();
    let mut seed = String::new();
    for _ in 0..10 {
        seed.push(rng.gen_range(0..=9).to_string().chars().next().unwrap());
    }
    init_input_system_with_default::<MapSeedInput>(&seed, commands, map_seed_query);
}

pub fn init_eid_percentage_input_system(
    commands: Commands,
    eid_percentage_query: Query<(Entity, &mut Text, &mut EidPercentageInput), Without<Initialized>>,
) {
    init_input_system_with_default::<EidPercentageInput>("WIP", commands, eid_percentage_query);
}

pub fn exit_system(mut exit: EventWriter<AppExit>) {
    exit.send(AppExit);
}