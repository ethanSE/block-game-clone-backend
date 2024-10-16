//! Functions and types for Rust (as WASM) <-> TS interop
use wasm_bindgen::prelude::wasm_bindgen;

use game_logic::{
    action::Action,
    game_mode,
    game_state::{self, GameState},
};

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
