mod board;
mod board_state;
mod game_mode;
mod game_state;
mod greedy_ai;
mod hand;
mod piece;
mod player;
mod player_hand_state;
mod player_state;
mod ts_interop;
mod utils;

use game_state::GameState;
use ts_interop::Action;
use wasm_bindgen::prelude::*;

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
