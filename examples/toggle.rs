use bevy::prelude::*;
use bevy_enum_filter::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Toggle Example".to_string(),
                resolution: (100., 100.).into(),
                ..default()
            }),
            ..default()
        }))
        // ! === Add the Filter === ! //
        .add_enum_filter::<Toggle>()
        // ! === Add the Filter === ! //
        .add_startup_system(info)
        .add_startup_system(spawn)
        .add_system(toggle)
        .add_system(on_state)
        .add_system(off_state)
        .run();
}

#[derive(Component, EnumFilter)]
enum Toggle {
    On,
    Off,
}

fn toggle(mut query: Query<&mut Toggle>, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::Space) {
        for mut state in &mut query {
            match *state {
                Toggle::On => *state = Toggle::Off,
                Toggle::Off => *state = Toggle::On,
            }
        }
    }
}

fn on_state(query: Query<Entity, Added<Enum!(Toggle::On)>>) {
    for _ in &query {
        println!("Entity is in `On` state. Press `Space` to toggle.");
    }
}

fn off_state(query: Query<Entity, Added<Enum!(Toggle::Off)>>) {
    for _ in &query {
        println!("Entity is in `Off` state. Press `Space` to toggle.");
    }
}

fn info() {
    println!("Press `Space` to toggle the entity");
}

fn spawn(mut commands: Commands) {
    commands.spawn(Toggle::Off);
}
