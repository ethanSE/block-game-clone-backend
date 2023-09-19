//! Contains [GameMode]

use serde::{Deserialize, Serialize};
extern crate alloc;
#[cfg(feature = "ts-interop")]
use {
    alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec},
    ts_rs::TS,
};

/// Represents game mode and map
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(tag = "type", content = "data")]
#[cfg_attr(
    feature = "ts-interop",
    derive(TS),
    ts(export, export_to = "pkg/types/GameMode.ts")
)]
pub enum GameMode {
    Solitaire(SolitaireMap),
    TwoPlayer(TwoPlayerMap),
    VSGreedyAI(TwoPlayerMap),
}

impl Default for GameMode {
    fn default() -> Self {
        Self::TwoPlayer(TwoPlayerMap::Tower)
    }
}

#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
#[cfg_attr(
    feature = "ts-interop",
    derive(TS),
    ts(export, export_to = "pkg/types/SolitaireMap.ts")
)]
pub enum SolitaireMap {
    FourByFiveByTwo,
}
#[derive(Serialize, Deserialize, Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(
    feature = "ts-interop",
    derive(TS),
    ts(export, export_to = "pkg/types/TwoPlayerMap.ts")
)]
pub enum TwoPlayerMap {
    Tower,
    Pyramid,
    Stairs,
    Wall,
}
