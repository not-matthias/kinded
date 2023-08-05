//! # Kinded
//!
//! Generate Rust enum kind types without boilerplate.
//!
//! ## Get Started
//!
//! ```
//! use kinded::Kinded;
//!
//! #[derive(Kinded)]
//! enum Drink {
//!     Mate,
//!     Coffee(String),
//!     Tea { variety: String, caffeine: bool }
//! }
//!
//! let drink = Drink::Coffee("Espresso".to_owned());
//! assert_eq!(drink.kind(), DrinkKind::Coffee);
//! ```
//!
//! Note, the definition of `DrinkKind` enum is generated automatically as well as `Drink::kind()` method.
//! To put it simply you get something similar to the following:
//!
//! ```ignore
//! #[derive(Debug, Clone, Copy, PartialEq, Eq)]
//! enum DrinkKind {
//!     Mate,
//!     Coffee,
//!     Tea
//! }
//!
//! impl Drink {
//!     fn kind(&self) -> DrinkKind {
//!         Drink::Mate => DrinkKind::Mate,
//!         Drink::Coffee(..) => DrinkKind::Coffee,
//!         Drink::Tea { .. } => DrinkKind::Tea,
//!     }
//! }
//!
//! ## Kinded trait
//!
//! The library provides `Kinded` trait:
//!
//! ```rs
//! pub trait Kinded {
//!     type Kind: PartialEq + Eq + Debug + Clone + Copy;
//!
//!     fn kind(&self) -> Self::Kind;
//! }
//! ```
//!
//! From the example above, the derived implementation of `Kinded` for `Drink` resembles the following:
//!
//! ```ignore
//! impl Kinded for Drink {
//!     type Kind = DrinkKind;
//!
//!     fn kind(&self) -> DrinkKind { /* implementation */ }
//! }
//! ```
//!
//! The `Kinded` trait allows to build abstract functions that can be used with different enum types.
//!
//! ## Iterating
//!
//! The kind type gets implementation of `::all()` associated function, which returns an iterator over all kind variants.
//! For example:
//!
//! ```
//! use kinded::Kinded;
//!
//! #[derive(Kinded)]
//! enum Drink {
//!     Mate,
//!     Coffee(String),
//!     Tea { variety: String, caffeine: bool }
//! }
//!
//! let all_drink_kinds: Vec<_> = DrinkKind::all().collect();
//! assert_eq!(all_drink_kinds, vec![DrinkKind::Mate, DrinkKind::Coffee, DrinkKind::Tea]);
//! ```
//!
//! ## Attributes
//!
//! ### Custom kind type name
//!
//! By default the kind type name is generated by adding postfix `Kind` to the original enum name.
//! This can be customized with `kind = ` attribute:
//!
//! ```
//! use kinded::Kinded;
//!
//! #[derive(Kinded)]
//! #[kinded(kind = SimpleDrink)]
//! enum Drink {
//!     Mate,
//!     Coffee(String),
//!     Tea { variety: String, caffeine: bool }
//! }
//!
//! assert_eq!(Drink::Mate.kind(), SimpleDrink::Mate);
//! ```
//!
//! ### Derive traits
//!
//! By default the kind type implements the following traits: `Debug`, `Clone`, `Copy`, `PartialEq`, `Eq`, `From<T>`, `From<&T>`.
//!
//! Extra traits can be derived with `derive(..)` attribute:
//!
//! ```
//! use kinded::Kinded;
//! use std::collections::HashSet;
//!
//! #[derive(Kinded)]
//! #[kinded(derive(Hash))]
//! enum Drink {
//!     Mate,
//!     Coffee(String),
//!     Tea { variety: String, caffeine: bool }
//! }
//!
//! let mut drink_kinds = HashSet::new();
//! drink_kinds.insert(DrinkKind::Mate);
//! ```
//!
//! ### Customize Display trait
//!
//! Implementation of `Display` trait can be customized in the `serde` fashion:
//!
//! ```
//! use kinded::Kinded;
//!
//! #[derive(Kinded)]
//! #[kinded(display = "snake_case")]
//! enum Drink {
//!     VeryHotBlackTea,
//!     Milk { fat: f64 },
//! }
//!
//! let tea = DrinkKind::VeryHotBlackTea;
//! assert_eq!(tea.to_string(), "very_hot_black_tea");
//! ```
//!
//! The possible values are `"snake_case"`, `"camelCase"`, `"PascalCase"`, `"SCREAMING_SNAKE_CASE"`, `"kebab-case"`, `"SCREAMING-KEBAB-CASE"`, `"Title Case"`, `"lowercase"`, `"UPPERCASE"`.
//!
//! ## A note about the war in Ukraine 🇺🇦
//!
//! Today I live in Berlin, I have the luxury to live a physically safe life.
//! But I am Ukrainian. The first 25 years of my life I spent in [Kharkiv](https://en.wikipedia.org/wiki/Kharkiv),
//! the second-largest city in Ukraine, 60km away from the border with russia. Today about [a third of my home city is destroyed](https://www.youtube.com/watch?v=ihoufBFSZds) by russians.
//! My parents, my relatives and my friends had to survive the artillery and air attack, living for over a month in basements.
//!
//! Some of them have managed to evacuate to EU. Some others are trying to live "normal lifes" in Kharkiv, doing there daily duties.
//! And some are at the front line right now, risking their lives every second to protect the rest.
//!
//! I encourage you to donate to [Charity foundation of Serhiy Prytula](https://prytulafoundation.org/en).
//! Just pick the project you like and donate. This is one of the best-known foundations, you can watch a [little documentary](https://www.youtube.com/watch?v=VlmWqoeub1Q) about it.
//! Your contribution to the Ukrainian military force is a contribution to my calmness, so I can spend more time developing the project.
//!
//! Thank you.
//!
//!
//! ## License
//!
//! MIT © [Serhii Potapov](https://www.greyblake.com)

pub use kinded_macros::Kinded;
use std::fmt::Debug;

pub trait Kinded {
    type Kind: PartialEq + Eq + Debug + Clone + Copy;

    fn kind(&self) -> Self::Kind;
}
