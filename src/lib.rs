#![doc = include_str!("../README.md")]

mod extensions;
mod filter_trait;
mod systems;

pub use bevy_enum_filter_derive::{Enum, EnumFilter};
pub use filter_trait::EnumFilter;
pub use systems::watch_for_enum;

pub mod prelude {
    pub use super::extensions::AddEnumFilter;
    pub use super::filter_trait::EnumFilter;
    pub use bevy_enum_filter_derive::{Enum, EnumFilter};
}

#[cfg(test)]
mod tests {
    use super::prelude::*;
    use bevy::prelude::*;
    use bevy_app::PostStartup;

    #[derive(Component, EnumFilter)]
    enum TestEnum {
        Unit,
        Tuple(usize),
        Struct { _foo: usize },
    }

    #[test]
    fn should_use_enum_filter() {
        #[derive(Event)]
        struct FoundFilter(bool);

        let mut app = App::new();
        app.add_enum_filter::<TestEnum>();
        app.add_event::<FoundFilter>();
        app.add_systems(Update, remove_entity_with_enum);

        let entity = app.world.spawn_empty().id();

        app.update();

        assert!(app.world.get_entity(entity).is_some());

        app.world
            .entity_mut(entity)
            .insert(TestEnum::Struct { _foo: 123 });

        app.update();
        app.update();

        assert!(app.world.get_entity(entity).is_none());

        fn remove_entity_with_enum(
            query: Query<Entity, With<Enum!(TestEnum::Struct)>>,
            mut commands: Commands,
        ) {
            for entity in &query {
                commands.entity(entity).despawn();
            }
        }
    }

    #[test]
    fn should_automatically_handle_markers() {
        let mut app = App::new();
        app.add_enum_filter::<TestEnum>();
        let entity = app.world.spawn_empty().id();

        app.update();

        let total_components = app.world.entity(entity).archetype().components().count();
        assert_eq!(0, total_components);
        assert!(!app
            .world
            .entity(entity)
            .contains::<test_enum_filters::Tuple>());

        app.world.entity_mut(entity).insert(TestEnum::Tuple(123));

        app.update();

        let total_components = app.world.entity(entity).archetype().components().count();
        assert_eq!(2, total_components);
        assert!(app
            .world
            .entity(entity)
            .contains::<test_enum_filters::Tuple>());

        app.world.entity_mut(entity).insert(TestEnum::Unit);

        app.update();

        let total_components = app.world.entity(entity).archetype().components().count();
        assert_eq!(2, total_components);
        assert!(!app
            .world
            .entity(entity)
            .contains::<test_enum_filters::Tuple>());
        assert!(app
            .world
            .entity(entity)
            .contains::<test_enum_filters::Unit>());
    }

    #[test]
    fn should_access_query_on_first_frame() {
        let mut app = App::new();
        app.add_enum_filter::<TestEnum>();

        let entity_unit = app.world.spawn_empty().id();
        let entity_tuple = app.world.spawn_empty().id();
        let entity_struct = app.world.spawn_empty().id();
        app.world.entity_mut(entity_unit).insert(TestEnum::Unit);
        app.world.entity_mut(entity_tuple).insert(TestEnum::Tuple(123));
        app.world.entity_mut(entity_struct).insert(TestEnum::Struct { _foo: 123 });

        let total_unit_components = app.world.query::<&Enum!(TestEnum::Unit)>().iter(&app.world).len();
        let total_tuple_components = app.world.query::<&Enum!(TestEnum::Tuple)>().iter(&app.world).len();
        let total_struct_components = app.world.query::<&Enum!(TestEnum::Struct)>().iter(&app.world).len();
        assert_eq!(0, total_unit_components);
        assert_eq!(0, total_tuple_components);
        assert_eq!(0, total_struct_components);

        // Ensure that the marker struct is created, mimicking how a real loop
        // would work.
        // Calling app.update() would run _all_ schedules, but we want to test
        // that we have created the marker struct before the `Update` schedule
        // runs.
        app.world.run_schedule(PostStartup);

        // We are now between the `PostStartup` schedule and `First` schedules.
        // This is important as we need to check that the marker struct was
        // created before the Update loop begins.
        // This means that we can call `query.single()` or `query.single_mut()`
        // without having to first check `query.is_empty()`, as the first
        // iteration of `Update` will have a populated query, if it matches.

        let total_unit_components = app.world.query::<&Enum!(TestEnum::Unit)>().iter(&app.world).len();
        let total_tuple_components = app.world.query::<&Enum!(TestEnum::Tuple)>().iter(&app.world).len();
        let total_struct_components = app.world.query::<&Enum!(TestEnum::Struct)>().iter(&app.world).len();
        assert_eq!(1, total_unit_components);
        assert_eq!(1, total_tuple_components);
        assert_eq!(1, total_struct_components);
    }
}
