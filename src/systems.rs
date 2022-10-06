use crate::filter_trait::EnumFilter;
use bevy_ecs::prelude::{Changed, Commands, Entity, Query};

/// A system that watches for changes to the given enum component.
///
/// Normally, you will not need to use this directly and can just use the [`add_enum_filter`] method on [`App`].
///
/// [`add_enum_filter`]: crate::extensions::AddEnumFilter::add_enum_filter
/// [`App`]: bevy_app::App
pub fn watch_for_enum<T: EnumFilter>(
    mut commands: Commands,
    query: Query<(Entity, &T), Changed<T>>,
) {
    for (entity, value) in &query {
        T::set_marker(&mut commands.entity(entity), value);
    }
}
