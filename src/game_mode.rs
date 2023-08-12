use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Copy)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "pkg/types/GameMode.ts")]
pub enum GameMode {
    Solitaire(SolitaireMap),
    TwoPlayer(TwoPlayerMap),
}

impl Default for GameMode {
    fn default() -> Self {
        Self::TwoPlayer(TwoPlayerMap::FourByFiveByFour)
    }
}

#[derive(Serialize, Deserialize, TS, Clone, Copy)]
#[serde(tag = "type")]
#[ts(export, export_to = "pkg/types/SolitaireMap.ts")]
pub enum SolitaireMap {
    FourByFiveByTwo,
}
#[derive(Serialize, Deserialize, TS, Clone, Copy)]
#[serde(tag = "type")]
#[ts(export, export_to = "pkg/types/TwoPlayerMap.ts")]
pub enum TwoPlayerMap {
    FourByFiveByFour,
    Pyramid,
}
