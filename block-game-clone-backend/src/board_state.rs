//! Contains [BoardState]

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::{
    board::{Board, Cube, CubeError},
    game_mode::GameMode,
    piece::Piece,
    player::Player,
};
/// The state of the board, including move preview
#[derive(Serialize, Deserialize, Default, Debug, TS, Clone)]
#[ts(export, export_to = "pkg/types/BoardState.ts")]
pub struct BoardState {
    /// the current state of the board, available space, pieces that are in play
    pub board: Board,
    /// used to show a player the result of a possible move, move validity
    pub previewed_piece: Option<Vec<Cube>>,
}

impl BoardState {
    pub fn new(game_mode: GameMode) -> Self {
        Self {
            board: Board::new(game_mode),
            previewed_piece: None,
        }
    }

    /// Previews a piece placement
    ///
    /// Used for allowing a user to visualize the effect of placing a piece on the board
    ///
    /// Contains information about move validity
    pub fn preview_piece(&mut self, current_player: Player, piece: Piece, position: Vector3<f32>) {
        self.previewed_piece =
            Some(self.check_piece_placement(current_player, piece.clone(), position));
    }

    ///  performs in bounds and collision checks
    ///
    ///  returns copy with optional error embedded
    fn check_in_bounds_no_collision(&self, preview_cube: &Cube) -> Cube {
        Cube {
            error: self
                .board
                .check_in_bounds_no_collision(preview_cube.position),
            ..*preview_cube
        }
    }

    /// Checks that a given cube of the potential play is supported either by a piece already in play or by another cube in the same piece
    ///
    /// adds CubeError::Unsupported if not
    fn check_cube_supported(&self, preview_cube: Cube, moved_piece: &Piece) -> Cube {
        if self.board.supports(&preview_cube.position)
            || moved_piece.supports(&preview_cube.position)
        {
            preview_cube
        } else {
            Cube {
                error: Some(CubeError::Unsupported),
                ..preview_cube
            }
        }
    }

    /// returns Vec of cubes with position and possible error information
    pub fn check_piece_placement(
        &self,
        current_player: Player,
        piece: Piece,
        position: Vector3<f32>,
    ) -> Vec<Cube> {
        // build the piece from the piece and position offset
        let moved_piece = piece.get_moved_copy(position);

        let preview_cubes: Vec<Cube> = moved_piece
            .coords
            .iter()
            .map(|v3| Cube {
                player: current_player,
                position: *v3,
                error: None,
            })
            .collect();

        match self.board.check_touches_piece(preview_cubes) {
            Ok(cubes) => cubes
                .iter()
                .map(|preview_cube| Self::check_in_bounds_no_collision(self, preview_cube))
                .map(|preview_cube| Self::check_cube_supported(self, preview_cube, &moved_piece))
                .collect(),
            Err(cubes) => cubes,
        }
    }

    /// Plays the selected piece if the previewed move is valid
    pub fn play_selected_piece(&mut self) -> Result<(), ()> {
        if let Some(preview_cubes) = &self.previewed_piece {
            if preview_cubes.iter().all(|c| c.error.is_none()) {
                self.board.add_cubes(preview_cubes);
                self.previewed_piece = None;
                Ok(())
            } else {
                Err(())
            }
        } else {
            Err(())
        }
    }

    /// Clears the previewed piece
    pub fn clear_previewed_piece(&mut self) {
        self.previewed_piece = None;
    }

    /// Returns the current score
    pub fn calculate_score(&self) -> HashMap<Player, i8> {
        self.board.calculate_score()
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        board::CubeError,
        game_state::GameState,
        piece::PieceName,
        ts_interop::{Action, V3},
    };
    use nalgebra::Vector3;

    #[test]
    fn test_supported_y_0() {
        let mut gs = GameState::default();
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));

        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));

        if let Some(preview_piece) = gs.board_state.previewed_piece {
            assert!(preview_piece.iter().all(|c| c.error.is_none()));
        }
    }

    #[test]
    fn test_supported_other_piece() {
        let mut gs = GameState::default();
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));

        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));

        if let Some(preview_piece) = &gs.board_state.previewed_piece {
            assert!(preview_piece.iter().all(|c| c.error.is_none()));
        }

        gs.apply_action(Action::PlayPreviewedPiece);

        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));

        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 1.0, 0.0))));

        if let Some(a) = &gs.board_state.previewed_piece {
            assert!(a.iter().all(|c| c.error.is_none()));
        }
    }

    #[test]
    fn test_supported_unsupported() {
        let mut gs = GameState::default();
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));

        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 1.0, 0.0))));

        if let Some(preview_piece) = &gs.board_state.previewed_piece {
            assert!(preview_piece
                .iter()
                .any(|c| c.error == Some(CubeError::Unsupported)));
        }
    }

    /// attempts to place two pieces in the same position, should fail
    #[test]
    fn collision() {
        let mut gs = GameState::default();

        // place piece
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));
        gs.apply_action(Action::PlayPreviewedPiece);

        //attempts to place another piece at the same position
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));

        if let Some(preivew_cubes) = &gs.board_state.previewed_piece {
            println!("{:?}", preivew_cubes);
            assert!(preivew_cubes.iter().any(|cube| cube.error.is_some()));
        }
    }
}
