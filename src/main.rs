use bevy::prelude::*;
use bevy::log;
use board_plugin::BoardPlugin;
use board_plugin::resources::{Board, BoardAssets, BoardOptions, SpriteMaterial};
#[cfg(feature = "debug")]
use board_plugin::components::Coordinates;

#[cfg(feature = "debug")]
use bevy_inspector_egui::quick::WorldInspectorPlugin;

#[derive(Debug, Clone, Eq, PartialEq, Hash, Default, States)]
pub enum AppState {
    #[default]
    Load,
    Reload,
    InGame,
    Pause,
    Out,
}

fn main() {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins.set( WindowPlugin {
            primary_window: Some(Window {
                title: "踩地雷".into(),
                resolution: (650., 650.).into(),
                ..Default::default()
            }),
            ..Default::default()
        }))
        .add_state::<AppState>()
        .add_systems(Update, state_handler)
        .add_plugins(BoardPlugin{ 
            loading_state: AppState::Load,
            reloading_state: AppState::Reload,
            running_state: AppState::InGame,
            pausing_state: AppState::Pause,
            exiting_state: AppState::Out,
        })
        .add_systems(Startup, setup_board);
    
    #[cfg(feature = "debug")]
    app.add_plugins(WorldInspectorPlugin::new());
    #[cfg(feature = "debug")]
    app.register_type::<Coordinates>();
    app.add_systems(Startup, camera_setup);
    app.run();
}

fn camera_setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn state_handler(
    current_state: Res<State<AppState>>, 
    mut next_state: ResMut<NextState<AppState>>,
    board: Option<Res<Board>>,
    keys: Res<Input<KeyCode>>,
) {
    if (current_state.get() == &AppState::Load 
        || current_state.get() == &AppState::Reload)
        && board.is_some() {
        next_state.set(AppState::InGame);
    }
    
    if keys.just_pressed(KeyCode::C) {
        log::debug!("clearing detected");
        if current_state.get() == &AppState::InGame {
            log::info!("clearing game");
            next_state.set(AppState::Out);
        }
    }

    if keys.just_pressed(KeyCode::G) {
        log::debug!("loading detected");
        if current_state.get() == &AppState::InGame {
            log::info!("re-loading game");
            next_state.set(AppState::Reload);
        }
        else if current_state.get() == &AppState::Out {
            log::info!("loading game");
            next_state.set(AppState::Load);
        }
    }

    if keys.just_pressed(KeyCode::Escape) {
        log::debug!("esc key detected");
        if current_state.get() == &AppState::InGame {
            log::info!("pause the game");
            next_state.set(AppState::Pause);
        }
        else if current_state.get() == &AppState::Pause {
            log::info!("resume the game");
            next_state.set(AppState::InGame);
        }
    }
}

fn setup_board(
    mut commands: Commands,
    mut next_state: ResMut<NextState<AppState>>,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(BoardOptions {
        map_size: (11, 11),
        bomb_count: 15,
        tile_padding: 3.,
        safe_start: true,
        ..Default::default()
    });

    commands.insert_resource(BoardAssets {
        label: "Default".to_string(),
        board_material: SpriteMaterial { 
            color: Color::WHITE, ..Default::default() 
        },
        tile_material: SpriteMaterial { 
            color: Color::DARK_GRAY, ..Default::default() 
        },
        covered_tile_material: SpriteMaterial { 
            color: Color::GRAY, ..Default::default() 
        },
        bomb_counter_font: asset_server.load("fonts/pixeled.ttf"),
        bomb_counter_colors: BoardAssets::default_colors(),
        flag_material:  SpriteMaterial { 
            color: Color::WHITE,
            texture: asset_server.load("sprites/flag.png"),
        },
        bomb_material:  SpriteMaterial { 
            color: Color::WHITE,
            texture: asset_server.load("sprites/bomb.png"),
        },
    });
    next_state.set(AppState::InGame);
}
