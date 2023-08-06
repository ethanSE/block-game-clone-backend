mod utils;

use board::Board;
use board_state::BoardState;
use game_state::GameState;
use nalgebra::Vector3;
use piece::PieceName;
use player_state::PlayerState;
use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
mod board;
mod board_state;
mod game_state;
mod hand;
mod piece;
mod player;
mod player_hand_state;
mod player_state;
use ts_rs::TS;

#[wasm_bindgen]
extern "C" {
    // Use `js_namespace` here to bind `console.log(..)` instead of just
    // `log(..)`
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);

    // The `console.log` is quite polymorphic, so we can bind it with multiple
    // signatures. Note that we need to use `js_name` to ensure we always call
    // `log` in JS.
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_u32(a: u32);

    // Multiple arguments too!
    #[wasm_bindgen(js_namespace = console, js_name = log)]
    fn log_many(a: &str, b: &str);
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

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/V3.ts")]
pub struct V3(#[ts(type = "[number, number,number]")] Vector3<i8>);

#[wasm_bindgen]
pub fn new_two_player_four_by_five() -> String {
    let gs: GameState = GameState {
        player_state: PlayerState::default(),
        board_state: BoardState {
            pieces: Board::new_four_by_five_by_four(),
            previewed_piece: None,
        },
    };

    serde_json::to_string(&gs).unwrap()
}

#[wasm_bindgen]
pub fn new_two_player_pyramid() -> String {
    let gs: GameState = GameState {
        player_state: PlayerState::default(),
        board_state: BoardState {
            pieces: Board::new_pyramid(),
            previewed_piece: None,
        },
    };

    serde_json::to_string(&gs).unwrap()
}

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
