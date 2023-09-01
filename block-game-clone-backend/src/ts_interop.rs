//! Functions and types for Rust (as WASM) <-> TS interop

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use wasm_bindgen::prelude::wasm_bindgen;

/// used for generating a new game from WASM
///
/// Takes in a game mode as an &str
#[wasm_bindgen]
pub fn new_game(game_mode_str: &str) -> String {
    let game_mode = serde_json::from_str::<game_mode::GameMode>(game_mode_str);

    match game_mode {
        Ok(gs) => serde_json::to_string(&GameState::new(gs)).unwrap(),
        _ => "Error".to_string(),
    }
}

/// Given a GameState and Action as &str's in WASM, returns the resulting GameState (as String)
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
/// A newtype wrapper around a [`nalgebra::Vector3<f32>`]
///
/// Allows for defining how the type should be serialized and deserialized as well as how the TypeScript type signature should be generated
#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/V3.ts")]
pub struct V3(#[ts(type = "[number, number,number]")] pub Vector3<f32>);

/// Action enum defines all actions that could be performed by a player
///
/// Used in [GameState::apply_action]
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

/// Which axis the current player has chosen to rotate the currently selected piece around
///
/// Used in [Action::RotateSelectedPiece]
#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/RotationAxis.ts")]
pub enum RotationAxis {
    X,
    Y,
}
