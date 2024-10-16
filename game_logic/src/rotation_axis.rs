use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// Which axis the current player has chosen to rotate the currently selected piece around
///
/// Used in [Action::RotateSelectedPiece]
#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/RotationAxis.ts")]
pub enum RotationAxis {
    X,
    Y,
}
