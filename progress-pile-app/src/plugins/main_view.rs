use bevy::{
    app::AppExit,
    color::palettes::css::CRIMSON,
    ecs::spawn::{SpawnIter, SpawnWith},
    prelude::*,
};

use crate::{
    despawn_screen, 
    themes::dark::{self, NORMAL_BUTTON},
    DisplayQuality,
    AppState,
    Volume,
};

pub fn main_view_plugin(app: &mut App) {
    app
        .add_systems(OnEnter(AppState::MainView), main_view_setup)
        .add_systems(OnExit(AppState::MainView), despawn_screen::<OnMainViewScreen>)
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(AppState::MainView))
        )
        
        ;
}
#[derive(Component)]
struct OnMainViewScreen;

#[derive(Component)]
struct SelectedOption;


#[derive(Component)]
enum MainViewButtonAction {
    OpenMainMenu,
}

fn button_system(
    mut interaction_query: Query<(&Interaction, &mut BackgroundColor, Option<&SelectedOption>),(Changed<Interaction>, With<Button>)>
) {
    for (interaction, mut background_color, selected) in &mut interaction_query {
        *background_color = match (*interaction, selected) {
            (Interaction::Pressed, _) | (_, Some(_)) => dark::PRESSED_BUTTON.into(),
            (Interaction::Hovered, None) => dark::HOVERED_BUTTON.into(),
            (Interaction::None, None) => dark::NORMAL_BUTTON.into(),
        }
    }
} 

fn main_view_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let header_node = Node {
        width: Val::Percent(100.0),
        height: Val::Px(20.0),
        justify_content: JustifyContent::SpaceBetween,
        align_items: AlignItems::Center,
        ..default()
    };
    let body_node = Node{
        width: Val::Percent(100.0),
        height: Val::Auto,
        ..default()
    };
    let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        },
        OnMainViewScreen,
        children![(
            header_node,
            children![(
                Button,
                Node{
                    width: Val::Px(30.0),
                    ..default()
                },
                MainViewButtonAction::OpenMainMenu,
                children![(
                    ImageNode::new(wrench_icon),
                    Node::default()
                 )
                ]
                
            )]
        )]
    )
    );
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MainViewButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MainViewButtonAction::OpenMainMenu => {
                    game_state.set(AppState::Menu);
                }
            }
        }
    }
}
