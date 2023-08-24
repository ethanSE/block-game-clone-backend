use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    board_state::BoardState,
    game_mode::GameMode,
    greedy_ai,
    piece::{Piece, PieceName},
    player::Player,
    player_state::PlayerState,
    ts_interop::{RotationAxis, Score, V3},
    Action,
};

#[derive(Serialize, Deserialize, TS, Default, Clone, Debug)]
#[ts(export, export_to = "pkg/types/GameState.ts")]
pub struct GameState {
    pub player_state: PlayerState,
    pub board_state: BoardState,
    pub game_mode: GameMode,
    pub score: Score,
}

impl GameState {
    pub fn new(game_mode: GameMode) -> Self {
        Self {
            game_mode,
            player_state: PlayerState::default(),
            board_state: BoardState::new(game_mode),
            score: Score::default(),
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::SelectPiece(name) => self.select_piece(name),
            Action::ClearSelectedPiece => self.clear_selected_piece(),
            Action::SetSelectedPieceOrigin(V3(new_origin)) => {
                self.set_selected_piece_origin(new_origin)
            }
            Action::RotateSelectedPiece(rotation_axis) => self.rotate_selected_piece(rotation_axis),
            Action::PreviewPiece(V3(position)) => self.preview_piece(position),
            Action::PlayPreviewedPiece => {
                let _ = self.play_previewed_piece();
            }
            Action::PassTurn => self.pass_turn(),
            Action::Reset => self.reset(),
        }
    }

    fn select_piece(&mut self, piece_name: PieceName) {
        self.player_state.select_piece(piece_name);
        self.board_state.clear_previewed_piece()
    }

    fn clear_selected_piece(&mut self) {
        self.board_state.clear_previewed_piece();
        self.player_state.clear_selected_piece();
    }

    fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        self.player_state.rotate_selected_piece(rotation_axis);
        self.board_state.clear_previewed_piece()
    }

    fn set_selected_piece_origin(&mut self, new_origin: Vector3<f32>) {
        self.player_state.set_selected_piece_origin(new_origin)
    }

    fn preview_piece(&mut self, position: Vector3<f32>) {
        if let Some((current_player, piece)) = self.player_state.get_selected_piece() {
            self.board_state
                .preview_piece(current_player, piece, position)
        }
    }

    pub fn play_previewed_piece(&mut self) -> Result<(), ()> {
        if let Ok(()) = self.board_state.play_selected_piece() {
            println!("played piece");
            self.player_state.play_selected_piece();
            self.score = self.board_state.calculate_score();
            if let GameMode::VSGreedyAI(_) = self.game_mode {
                println!("p2 is current player and in greedy AI mode");
                if self.player_state.current_player == Player::P2 {
                    *self = greedy_ai::greedy_ai(self.clone());
                }
            }
            Ok(())
        } else {
            Err(())
        }
    }

    pub fn pass_turn(&mut self) {
        self.player_state.toggle_current_player();
        self.board_state.clear_previewed_piece();
        if let GameMode::VSGreedyAI(_) = self.game_mode {
            if self.player_state.current_player == Player::P2 {
                *self = greedy_ai::greedy_ai(self.clone());
            }
        }
    }

    fn reset(&mut self) {
        self.player_state = PlayerState::default();
        self.board_state = BoardState::default();
    }

    pub fn get_available_piece_rotations(&self) -> Vec<(PieceName, Piece)> {
        self.player_state.get_available_piece_rotations()
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::{
        game_mode::GameMode, game_state::GameState, piece::PieceName, ts_interop::V3, Action,
    };

    #[test]
    fn test_multiple_actions() {
        let mut gs = GameState::new(GameMode::default());

        gs.apply_action(Action::SelectPiece(PieceName::Corner));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));
        gs.apply_action(Action::PlayPreviewedPiece);

        let gs_str = serde_json::to_string(&gs).unwrap();
        println!("{}", gs_str)
    }
}
