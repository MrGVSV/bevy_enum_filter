use bevy_ecs::prelude::Component;
use bevy_ecs::system::EntityCommands;

/// A trait used to denote an enum as "filterable".
///
/// This should _never_ be manually implemented.
/// Instead, use the [`EnumFilter`](bevy_enum_filter_derive::EnumFilter) derive macro.
pub trait EnumFilter: Component {
    /// Set the marker on the given entity to the given enum value.
    fn set_marker(commands: &mut EntityCommands, value: &Self);
}
