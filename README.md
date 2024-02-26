# bevy_enum_filter

[![Crates.io](https://img.shields.io/crates/v/bevy_enum_filter)](https://crates.io/crates/bevy_enum_filter)
[![Docs](https://img.shields.io/docsrs/bevy_enum_filter)](https://docs.rs/bevy_enum_filter/) 
[![License](https://img.shields.io/crates/l/bevy_enum_filter)](./LICENSE.md) 

Filter queries by enum variants!

In Rust, enum variants *aren't* types. This means we normally can't filter for them in a Bevy `Query`. The alternative, then is to use a set of "marker" components. This works well enough, but we miss out on the semantics of using an enum (and the ability to perform enum-specific operations such as `match`-ing).

This crate allows us to query for entities based on the *specific* variant of an enum component:

```rust
use bevy::prelude::*;
use bevy_enum_filter::prelude::*;

#[derive(Component, EnumFilter)]
enum ItemType {
  Equippable(usize),
  Weapon(usize),
  Potion(usize)
}

fn apply_potion(item_query: Query<(Entity, &ItemType), Added<Enum!(ItemType::Potion)>>) {
  // ...
}

fn main() {
  App::new()
    // ...
    .add_enum_filter::<ItemType>()
    .add_systems(Update, apply_potion)
    .run()
}
```

## 📲 Installation

Add the following to the `[dependencies]` section of your `Cargo.toml`.

```text
bevy_enum_filter = "0.3.0"
```

## 🤨 How it works

*Surprise! It's just marker structs!*

Deriving `EnumFilter` generates a module containing a marker struct per variant. The module is named using the enum's name (snake-cased) followed by `_filters`. For example, our `ItemType` enum generates the following module:

```rust
// Auto-generated!
mod item_type_filters {
  #[derive(bevy::prelude::Component)]
  pub struct Equippable;
  #[derive(bevy::prelude::Component)]
  pub struct Weapon;
  #[derive(bevy::prelude::Component)]
  pub struct Potion;
}
```

When we then register our enum using `app.add_enum_filter`, we are adding a system that watches for changes (additions/mutations) related to that enum component. The system will then add or remove the appropriate marker struct whenever there's a change.

The `Enum!` macro then takes the given enum path and grabs the corresponding marker struct from the module. So `Enum!(ItemType::Potion)` corresponds to the `item_type_filters::Potion` type.

> 📢: This is why you *must* have your generated module in scope!

##### Caveats

Because this is basically just change detection under the hood, it's important to remember when the filter actually takes effect. By default, the system added by `app.add_enum_filter` runs in the `PostUpdate` stage. This means you will not see the filter work until the that stage is complete.

>  And remember that any components changed in `Update` would not see their filters work until `PostUpdate` anyway since we need to cross a stage boundary for commands to flush. This means we realistically only lose out on a single stage.

If you need it to run after a certain system or within a certain stage, you could always add the `watch_for_enum` system yourself.

## 🕊 Bevy Compatibility

| bevy   | bevy_enum_filter |
| :----- | ---------------- |
| 0.8.1  | 0.1.0            |
| 0.11.x | 0.2.0            |
| 0.12.x | 0.3.0            |
| 0.13.x | 0.4.0            |
