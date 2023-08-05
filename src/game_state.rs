use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    board_state::BoardState, piece::PieceName, player_state::PlayerState, Action, RotationAxis, V3,
};

#[derive(Serialize, Deserialize, Default, TS)]
#[ts(export, export_to = "pkg/types/GameState.ts")]
pub struct GameState {
    pub player_state: PlayerState,
    pub board_state: BoardState,
}

impl GameState {
    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::SelectPiece(name) => self.select_piece(name),
            Action::SetSelectedPieceOrigin(V3(new_origin)) => {
                self.set_selected_piece_origin(new_origin)
            }
            Action::RotateSelectedPiece(rotation_axis) => self.rotate_selected_piece(rotation_axis),
            Action::PreviewPiece(V3(position)) => self.preview_piece(position),
            Action::PlayPreviewedPiece => self.play_previewed_piece(),
            Action::PassTurn => self.pass_turn(),
            Action::Reset => self.reset(),
        }
    }

    fn select_piece(&mut self, piece_name: PieceName) {
        self.player_state.select_piece(piece_name);
        self.board_state.clear_previewed_piece()
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

    fn play_previewed_piece(&mut self) {
        if let Ok(()) = self.board_state.play_selected_piece() {
            self.player_state.play_selected_piece()
        }
    }

    fn pass_turn(&mut self) {
        self.player_state.toggle_current_player();
        self.board_state.clear_previewed_piece()
    }

    fn reset(&mut self) {
        self.player_state = PlayerState::default();
        self.board_state = BoardState::default();
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::{game_state::GameState, piece::PieceName, Action, V3};

    #[test]
    fn serialize() {
        let a = serde_json::to_string(&GameState::default()).unwrap();

        println!("{a}")
    }

    #[test]
    fn test_multiple_actions() {
        let mut gs = GameState::default();

        gs.apply_action(Action::SelectPiece(PieceName::Corner));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));
        gs.apply_action(Action::PlayPreviewedPiece);

        let gs_str = serde_json::to_string(&gs).unwrap();
        println!("{}", gs_str)
    }
}
