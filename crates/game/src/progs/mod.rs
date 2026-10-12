//! id's game logic, QuakeC 1.06 (`progs.src`'s files, in its order), translated by hand into
//! Rust. Every function keeps its QuakeC name and parameters; `self`, `other` and the other
//! globals live in `Game`. QuakeC's single namespace is kept: each file sees every other's
//! functions, as every QuakeC file did.

pub use crate::builtins::*;
pub use crate::*;
pub use glam::Vec3;

mod subs;
pub use subs::*;
mod fight;
pub use fight::*;
mod ai;
pub use ai::*;
mod combat;
pub use combat::*;
mod items;
pub use items::*;
mod weapons;
pub use weapons::*;
mod qc_world;
pub use qc_world::*;
mod client;
pub use client::*;
mod player;
pub use player::*;
mod monsters;
pub use monsters::*;
mod doors;
pub use doors::*;
mod buttons;
pub use buttons::*;
mod triggers;
pub use triggers::*;
mod plats;
pub use plats::*;
mod misc;
pub use misc::*;
mod ogre;
pub use ogre::*;
mod demon;
pub use demon::*;
mod shambler;
pub use shambler::*;
mod knight;
pub use knight::*;
mod soldier;
pub use soldier::*;
mod wizard;
pub use wizard::*;
mod dog;
pub use dog::*;
mod zombie;
pub use zombie::*;
mod boss;
pub use boss::*;
mod tarbaby;
pub use tarbaby::*;
mod hknight;
pub use hknight::*;
mod fish;
pub use fish::*;
mod shalrath;
pub use shalrath::*;
mod enforcer;
pub use enforcer::*;
mod oldone;
pub use oldone::*;

mod spawn_table;
pub use spawn_table::spawn_function;
