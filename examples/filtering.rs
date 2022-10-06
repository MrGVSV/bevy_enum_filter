use bevy::prelude::*;
use bevy_enum_filter::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 100.0,
            height: 100.0,
            title: "Filter Example".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // ! === Add the Filter === ! //
        .add_enum_filter::<Choice>()
        // ! === Add the Filter === ! //
        .add_startup_system(info)
        .add_system(spawn)
        .add_system(on_spawn_a)
        .add_system(on_spawn_b)
        .add_system(on_spawn_c)
        .run();
}

#[derive(Component, EnumFilter)]
enum Choice {
    A,
    B,
    C,
}

fn spawn(mut commands: Commands, input: Res<Input<KeyCode>>) {
    if input.just_pressed(KeyCode::A) {
        commands.spawn_bundle((Choice::A,));
    }
    if input.just_pressed(KeyCode::B) {
        commands.spawn_bundle((Choice::B,));
    }
    if input.just_pressed(KeyCode::C) {
        commands.spawn_bundle((Choice::C,));
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
