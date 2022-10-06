use bevy::prelude::*;
use bevy_enum_filter::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 100.0,
            height: 100.0,
            title: "Toggle Example".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
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

fn on_state(query: Query<Entity, Added<enum_filter!(Toggle::On)>>) {
    for _ in &query {
        println!("Entity is in `On` state. Press `Space` to toggle.");
    }
}

fn off_state(query: Query<Entity, Added<enum_filter!(Toggle::Off)>>) {
    for _ in &query {
        println!("Entity is in `Off` state. Press `Space` to toggle.");
    }
}

fn info() {
    println!("Press `Space` to toggle the entity");
}

fn spawn(mut commands: Commands) {
    commands.spawn().insert(Toggle::Off);
}
