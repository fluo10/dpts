use bevy::{
    app::AppExit,
    color::palettes::css::CRIMSON,
    ecs::spawn::{SpawnIter, SpawnWith},
    prelude::*,
};

use super::{
    DisplayQuality,
    GameState,
    Volume,
};

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum SettingsState {
    Main,
    #[default]
    Disabled,
}

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(GamsState::Settin), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), despawn_screen::<OnMainMenuScreen>)
        .add_ssytems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Setting)
        )
}