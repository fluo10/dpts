mod plugins;

use bevy::prelude::*;

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum AppState {
    #[default]
    MainView,
    Setting,
    List,
    Graph
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum DisplayQuality {
    Low,
    Medium,
    High,
}

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
struct Volume(u32);

#[derive(Resource, Debug, Component, PartialEq, Eq, Clone, Copy)]
enum ViewerMode {
    PokerTip2D,
    PokerTip3D,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .insert_resource(DisplayQuality::Medium)
        .insert_resource(Volume(7))
        .insert_resource(ViewerMode::PokerTip2D)
        .init_state::<AppState>()
        .add_systems(Startup, setup)
        .add_plugins(((plugins::main_menu::menu_plugin, plugins::main_view::main_view_plugin)))
        .run();

}
