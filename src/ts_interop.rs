use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::piece::PieceName;

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/V3.ts")]
pub struct V3(#[ts(type = "[number, number,number]")] pub Vector3<i8>);

#[derive(Serialize, Deserialize, TS)]
#[serde(tag = "type", content = "data")]
#[ts(export, export_to = "pkg/types/Action.ts")]
pub enum Action {
    SelectPiece(PieceName),
    SetSelectedPieceOrigin(V3),
    RotateSelectedPiece(RotationAxis),
    PreviewPiece(V3),
    PlayPreviewedPiece,
    PassTurn,
    Reset,
}

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/RotationAxis.ts")]
pub enum RotationAxis {
    X,
    Y,
}
