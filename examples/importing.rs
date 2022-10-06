use bevy::prelude::*;
use bevy_enum_filter::prelude::*;

fn main() {
    App::new()
        .insert_resource(WindowDescriptor {
            width: 100.0,
            height: 100.0,
            title: "Import Example".to_string(),
            ..default()
        })
        .add_plugins(DefaultPlugins)
        // ! === Add the Filter === ! //
        .add_enum_filter::<components::Choice>()
        // ! === Add the Filter === ! //
        .add_startup_system(systems::info)
        .add_system(systems::spawn)
        .add_system(systems::on_spawn_a)
        .add_system(systems::on_spawn_b)
        .add_system(systems::on_spawn_c)
        .run();
}

mod components {
    use bevy::prelude::*;
    use bevy_enum_filter::prelude::*;

    #[derive(Component, EnumFilter)]
    pub enum Choice {
        A,
        B,
        C,
    }
}

mod systems {
    use bevy::prelude::*;
    use bevy_enum_filter::prelude::*;

    // ! === Import Enum AND Filter Module === ! //
    use super::components::{choice_filters, Choice};
    // ! === Import Enum AND Filter Module === ! //

    pub fn spawn(mut commands: Commands, input: Res<Input<KeyCode>>) {
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

    pub fn on_spawn_a(query: Query<Entity, Added<enum_filter!(Choice::A)>>) {
        for _ in &query {
            println!("Spawned entity with `Choice::A`!");
        }
    }

    pub fn on_spawn_b(query: Query<Entity, Added<enum_filter!(Choice::B)>>) {
        for _ in &query {
            println!("Spawned entity with `Choice::B`!");
        }
    }

    pub fn on_spawn_c(query: Query<Entity, Added<enum_filter!(Choice::C)>>) {
        for _ in &query {
            println!("Spawned entity with `Choice::C`!");
        }
    }

    pub fn info() {
        println!(
            "Press any of the following keys to spawn an entity with that value: [`A`, `B`, or `C`]"
        );
    }
}
