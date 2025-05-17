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

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug, Hash, States)]
enum MenuState {
    Main,
    Settings,
    SettingsDisplay,
    SettingsVolume,
    #[default]
    Disabled,
}

pub fn menu_plugin(app: &mut App) {
    app
        .init_state::<MenuState>()
        .add_systems(OnEnter(AppState::Menu), menu_setup)
        .add_systems(OnEnter(MenuState::Main), main_menu_setup)
        .add_systems(OnExit(MenuState::Main), crate::despawn_screen::<OnMainMenuScreen>)
        .add_systems(OnEnter(MenuState::Settings), settings_menu_setup)
        .add_systems(
            OnExit(MenuState::Settings),
            crate::despawn_screen::<OnSettingsMenuScreen>
        )
        .add_systems(OnEnter(MenuState::SettingsDisplay), display_settings_menu_setup)
        .add_systems(
            Update,
            (setting_button::<DisplayQuality>.run_if(in_state(MenuState::SettingsDisplay)))
        )
        .add_systems(
            OnExit(MenuState::SettingsDisplay),
            despawn_screen::<OnDisplaySettingsMenuScreen>
        )
        .add_systems(OnEnter(MenuState::SettingsVolume), sound_settings_menu_setup)
        .add_systems(
            Update,
            (setting_button::<Volume>.run_if(in_state(MenuState::SettingsVolume)))
        )
        .add_systems(
            OnExit(MenuState::SettingsVolume),
            despawn_screen::<OnSoundSettingsMenuScreen>
        )
        .add_systems(
            Update,
            (menu_action, button_system).run_if(in_state(AppState::Menu))
        )
        
        ;
}
#[derive(Component)]
struct OnMainMenuScreen;

#[derive(Component)]
struct OnSettingsMenuScreen;

#[derive(Component)]
struct OnDisplaySettingsMenuScreen;

#[derive(Component)]
struct OnSoundSettingsMenuScreen;

#[derive(Component)]
struct SelectedOption;


#[derive(Component)]
enum MenuButtonAction {
    BackToMainView,
    BackToMainMenu,
    BackToSettings,
    Settings,
    SettingsDisplay,
    SettingsSound,
    Quit,
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
fn setting_button<T: Resource + Component + PartialEq + Copy> (
    interaction_query: Query<(&Interaction, &T, Entity), (Changed<Interaction>, With<Button>)>,
    selected_query: Single<(Entity, &mut BackgroundColor), With<SelectedOption>>,
    mut commands: Commands,
    mut setting: ResMut<T>
) {
    let (previous_button, mut previous_button_color) = selected_query.into_inner();
    for (interaction, button_setting, entity) in &interaction_query {
        if *interaction == Interaction::Pressed && *setting != *button_setting {
            *previous_button_color = NORMAL_BUTTON.into();
            commands.entity(previous_button).remove::<SelectedOption>();
            commands.entity(entity).insert(SelectedOption);
            *setting = *button_setting
        }
    }
}

fn menu_setup(mut menu_state: ResMut<NextState<MenuState>>) {
    menu_state.set(MenuState::Main);
}

fn main_menu_setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    let button_node = Node {
        width: Val::Px(300.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_icon_node = Node {
        width: Val::Px(30.0),
        position_type: PositionType::Absolute,
        left: Val::Px(10.0),
        ..default()
    };
    let button_text_font = TextFont {
        font_size: 33.0,
        ..default()
    };
    let right_icon = asset_server.load("textures/Game Icons/right.png");
    let wrench_icon = asset_server.load("textures/Game Icons/wrench.png");
    let exit_icon = asset_server.load("textures/Game Icons/exitRight.png");


    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnMainMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column, 
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(dark::BACKGROUND_COLOR.into()),
            children![
                (
                    Text::new("Progress Pile"),
                    TextFont {
                        font_size: 67.0,
                        ..default()
                    },
                    TextColor(dark::TEXT_COLOR),
                    Node {
                        margin: UiRect::all(Val::Px(50.0)),
                        ..default()
                    },
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(dark::NORMAL_BUTTON),
                    MenuButtonAction::BackToMainView,
                    children![
                        (ImageNode::new(right_icon), button_icon_node.clone()),
                        (
                            Text::new("Back to main window"),
                            button_text_font.clone(),
                            TextColor(dark::TEXT_COLOR),
                        )
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(dark::NORMAL_BUTTON),
                    MenuButtonAction::Settings,
                    children![
                        (ImageNode::new(wrench_icon), button_icon_node.clone()),
                        (
                            Text::new("Settings"),
                            button_text_font.clone(),
                            TextColor(dark::TEXT_COLOR),
                        )
                    ]
                ),
                (
                    Button,
                    button_node.clone(),
                    BackgroundColor(dark::NORMAL_BUTTON),
                    MenuButtonAction::Quit,
                    children![
                        (ImageNode::new(exit_icon), button_icon_node.clone()),
                        (
                            Text::new("Quit"),
                            button_text_font.clone(),
                            TextColor(dark::TEXT_COLOR),
                        )
                    ]
                ),
            ]
        )]
    ));

}

fn settings_menu_setup(mut commands: Commands) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(dark::TEXT_COLOR),
    );

    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            Children::spawn(SpawnIter(
                [
                    (MenuButtonAction::SettingsDisplay, "Display"),
                    (MenuButtonAction::SettingsSound, "Sound"),
                    (MenuButtonAction::BackToMainMenu, "Back"),
                ]
                .into_iter()
                .map(move |(action, text)| {
                    (
                        Button,
                        button_node.clone(),
                        BackgroundColor(NORMAL_BUTTON),
                        action,
                        children![(Text::new(text), button_text_style.clone())],
                    )
                })
            ))
        )],
    ));
}

fn display_settings_menu_setup(mut commands: Commands, display_quality: Res<DisplayQuality>) {
    fn button_node() -> Node {
        Node {
            width: Val::Px(200.0),
            height: Val::Px(65.0),
            margin: UiRect::all(Val::Px(20.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        }
    }
    fn button_text_style() -> impl Bundle {
        (
            TextFont {
                font_size: 33.0,
                ..default()
            },
            TextColor(dark::TEXT_COLOR),
        )
    }

    let display_quality = *display_quality;
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnDisplaySettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                // Create a new `Node`, this time not setting its `flex_direction`. It will
                // use the default value, `FlexDirection::Row`, from left to right.
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        // Display a label for the current setting
                        Spawn((Text::new("Display Quality"), button_text_style())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for quality_setting in [
                                DisplayQuality::Low,
                                DisplayQuality::Medium,
                                DisplayQuality::High,
                            ] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(150.0),
                                        height: Val::Px(65.0),
                                        ..button_node()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    quality_setting,
                                    children![(
                                        Text::new(format!("{quality_setting:?}")),
                                        button_text_style(),
                                    )],
                                ));
                                if display_quality == quality_setting {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                // Display the back button to return to the settings screen
                (
                    Button,
                    button_node(),
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style())]
                )
            ]
        )],
    ));
}

fn sound_settings_menu_setup(mut commands: Commands, volume: Res<Volume>) {
    let button_node = Node {
        width: Val::Px(200.0),
        height: Val::Px(65.0),
        margin: UiRect::all(Val::Px(20.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };
    let button_text_style = (
        TextFont {
            font_size: 33.0,
            ..default()
        },
        TextColor(dark::TEXT_COLOR),
    );

    let volume = *volume;
    let button_node_clone = button_node.clone();
    commands.spawn((
        Node {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            ..default()
        },
        OnSoundSettingsMenuScreen,
        children![(
            Node {
                flex_direction: FlexDirection::Column,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(CRIMSON.into()),
            children![
                (
                    Node {
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    BackgroundColor(CRIMSON.into()),
                    Children::spawn((
                        Spawn((Text::new("Volume"), button_text_style.clone())),
                        SpawnWith(move |parent: &mut ChildSpawner| {
                            for volume_setting in [0, 1, 2, 3, 4, 5, 6, 7, 8, 9] {
                                let mut entity = parent.spawn((
                                    Button,
                                    Node {
                                        width: Val::Px(30.0),
                                        height: Val::Px(65.0),
                                        ..button_node_clone.clone()
                                    },
                                    BackgroundColor(NORMAL_BUTTON),
                                    Volume(volume_setting),
                                ));
                                if volume == Volume(volume_setting) {
                                    entity.insert(SelectedOption);
                                }
                            }
                        })
                    ))
                ),
                (
                    Button,
                    button_node,
                    BackgroundColor(NORMAL_BUTTON),
                    MenuButtonAction::BackToSettings,
                    children![(Text::new("Back"), button_text_style)]
                )
            ]
        )],
    ));
}

fn menu_action(
    interaction_query: Query<
        (&Interaction, &MenuButtonAction),
        (Changed<Interaction>, With<Button>),
    >,
    mut app_exit_events: EventWriter<AppExit>,
    mut menu_state: ResMut<NextState<MenuState>>,
    mut game_state: ResMut<NextState<AppState>>,
) {
    for (interaction, menu_button_action) in &interaction_query {
        if *interaction == Interaction::Pressed {
            match menu_button_action {
                MenuButtonAction::Quit => {
                    app_exit_events.write(AppExit::Success);
                }
                MenuButtonAction::BackToMainView => {
                    game_state.set(AppState::MainView);
                    menu_state.set(MenuState::Disabled);
                }
                MenuButtonAction::Settings => menu_state.set(MenuState::Settings),
                MenuButtonAction::SettingsDisplay => {
                    menu_state.set(MenuState::SettingsDisplay);
                }
                MenuButtonAction::SettingsSound => {
                    menu_state.set(MenuState::SettingsVolume);
                }
                MenuButtonAction::BackToMainMenu => menu_state.set(MenuState::Main),
                MenuButtonAction::BackToSettings => {
                    menu_state.set(MenuState::Settings);
                }
            }
        }
    }
}
