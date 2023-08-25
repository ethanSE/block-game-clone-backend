use std::ops::Index;

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{piece::PieceName, player::Player};

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/V3.ts")]
pub struct V3(#[ts(type = "[number, number,number]")] pub Vector3<f32>);

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "pkg/types/Action.ts")]
pub enum Action {
    SelectPiece(PieceName),
    ClearSelectedPiece,
    SetSelectedPieceOrigin(V3),
    RotateSelectedPiece(RotationAxis),
    PreviewPiece(V3),
    PlayPreviewedPiece,
    PassTurn,
    Reset,
    MakeGreedyAIMove,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/RotationAxis.ts")]
pub enum RotationAxis {
    X,
    Y,
}

#[derive(Serialize, Deserialize, TS, Default, Clone, Debug)]
#[ts(export, export_to = "pkg/types/Score.ts")]
pub struct Score {
    pub p1: i8,
    pub p2: i8,
}

impl Index<Player> for Score {
    type Output = i8;
    fn index(&self, player: Player) -> &Self::Output {
        match player {
            Player::P1 => &self.p1,
            Player::P2 => &self.p2,
        }
    }
}
