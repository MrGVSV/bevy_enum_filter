use crate::filter_trait::EnumFilter;
use crate::systems::watch_for_enum;
use bevy_app::{App, CoreSet};
use bevy_ecs::schedule::IntoSystemConfig;

/// Extension trait for [`App`] that enables adding an [enum filter].
///
/// [enum filter]: crate::EnumFilter
pub trait AddEnumFilter {
    /// Register an enum filter.
    ///
    /// This will internally add a system to the [`PostUpdate`] stage that finds all entities with
    /// a component of `T` and automatically manage their respective markers.
    ///
    /// [`PostUpdate`]: CoreStage::PostUpdate
    fn add_enum_filter<T: EnumFilter>(&mut self) -> &mut Self;
}

impl AddEnumFilter for App {
    fn add_enum_filter<T: EnumFilter>(&mut self) -> &mut Self {
        self.add_system(watch_for_enum::<T>.in_base_set(CoreSet::PostUpdate))
    }
}
