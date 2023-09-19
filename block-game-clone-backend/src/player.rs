//! Contains [Player] enum
use serde::{Deserialize, Serialize};
extern crate alloc;

#[cfg(feature = "ts-interop")]
use {
    alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec},
    ts_rs::TS,
};

/// enum representing a player
///
/// piece ownership, indexing into structures
#[derive(
    Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Default, Debug, Hash, PartialOrd, Ord,
)]
#[cfg_attr(
    feature = "ts-interop",
    derive(TS),
    ts(export, export_to = "pkg/types/Player.ts")
)]
pub enum Player {
    #[default]
    P1,
    P2,
}

impl Player {
    /// toggles between values
    pub fn get_other(&self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}
