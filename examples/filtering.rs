use bevy::prelude::*;
use bevy_enum_filter::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Filter Example".to_string(),
                resolution: (100., 100.).into(),
                ..default()
            }),
            ..default()
        }))
        // ! === Add the Filter === ! //
        .add_enum_filter::<Choice>()
        // ! === Add the Filter === ! //
        .add_systems(Startup, info)
        .add_systems(Update, (spawn, on_spawn_a, on_spawn_b, on_spawn_c))
        .run();
}

#[derive(Component, EnumFilter)]
enum Choice {
    A,
    B,
    C,
}

fn spawn(mut commands: Commands, input: Res<ButtonInput<KeyCode>>) {
    if input.just_pressed(KeyCode::KeyA) {
        commands.spawn((Choice::A,));
    }
    if input.just_pressed(KeyCode::KeyB) {
        commands.spawn((Choice::B,));
    }
    if input.just_pressed(KeyCode::KeyC) {
        commands.spawn((Choice::C,));
    }
}

fn on_spawn_a(query: Query<Entity, Added<Enum!(Choice::A)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::A`!");
    }
}

fn on_spawn_b(query: Query<Entity, Added<Enum!(Choice::B)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::B`!");
    }
}

fn on_spawn_c(query: Query<Entity, Added<Enum!(Choice::C)>>) {
    for _ in &query {
        println!("Spawned entity with `Choice::C`!");
    }
}

fn info() {
    println!(
        "Press any of the following keys to spawn an entity with that value: [`A`, `B`, or `C`]"
    );
}
