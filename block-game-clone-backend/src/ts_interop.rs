use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;

#[wasm_bindgen]
pub fn new_game(game_mode_str: &str) -> String {
    let game_mode = serde_json::from_str::<game_mode::GameMode>(game_mode_str);

    match game_mode {
        Ok(gs) => serde_json::to_string(&GameState::new(gs)).unwrap(),
        _ => "Error".to_string(),
    }
}

#[wasm_bindgen]
pub fn next_game_state(current_state_s: &str, action_s: &str) -> String {
    let current_state = serde_json::from_str::<game_state::GameState>(current_state_s);
    let action = serde_json::from_str::<Action>(action_s);

    match (current_state, action) {
        (Ok(mut cs), Ok(action)) => {
            cs.apply_action(action);
            serde_json::to_string(&cs).unwrap()
        }
        _ => "invalid".to_string(),
    }
}

use crate::{
    game_mode,
    game_state::{self, GameState},
    piece::PieceName,
};

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
