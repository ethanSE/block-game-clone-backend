use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, PartialEq, Eq, Copy, Clone, Default, Debug, TS, Hash)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = "pkg/types/Player.ts")]
pub enum Player {
    #[default]
    P1,
    P2,
}

impl Player {
    pub fn get_other(&self) -> Self {
        match self {
            Player::P1 => Player::P2,
            Player::P2 => Player::P1,
        }
    }
}
