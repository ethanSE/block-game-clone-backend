//! Contains [GameMode]

use serde::{Deserialize, Serialize};
use ts_rs::TS;
extern crate alloc;
use alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec};

/// Represents game mode and map
#[derive(Serialize, Deserialize, TS, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "pkg/types/GameMode.ts")]
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

#[derive(Serialize, Deserialize, TS, Clone, Copy, PartialEq, Eq, Debug)]
#[serde(tag = "type")]
#[ts(export, export_to = "pkg/types/SolitaireMap.ts")]
pub enum SolitaireMap {
    FourByFiveByTwo,
}
#[derive(Serialize, Deserialize, TS, Clone, Copy, PartialEq, Eq, Debug)]
#[ts(export, export_to = "pkg/types/TwoPlayerMap.ts")]
pub enum TwoPlayerMap {
    Tower,
    Pyramid,
    Stairs,
    Wall,
}
