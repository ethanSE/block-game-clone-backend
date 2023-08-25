use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    board::{Board, Cube, CubeError},
    game_mode::GameMode,
    piece::Piece,
    player::Player,
    ts_interop::Score,
};

#[derive(Serialize, Deserialize, Default, Debug, TS, Clone)]
#[ts(export, export_to = "pkg/types/BoardState.ts")]
pub struct BoardState {
    pub board: Board,
    pub previewed_piece: Option<Vec<Cube>>,
}

impl BoardState {
    pub fn new(game_mode: GameMode) -> Self {
        Self {
            board: Board::new(game_mode),
            previewed_piece: None,
        }
    }

    pub fn preview_piece(&mut self, current_player: Player, piece: Piece, position: Vector3<f32>) {
        self.previewed_piece =
            Some(self.check_piece_placement(current_player, piece.clone(), position));
    }

    pub fn check_piece_placement(
        &self,
        current_player: Player,
        piece: Piece,
        position: Vector3<f32>,
    ) -> Vec<Cube> {
        //build the piece from the piece and position offset
        let moved_piece = piece.get_moved_copy(position);

        let positions: Vec<Cube> = moved_piece
            .coords
            .iter()
            .map(|v3| Cube {
                player: current_player,
                position: *v3,
                error: None,
            })
            .collect();

        let check_inbounds_no_collision = |c: &Cube| Cube {
            error: self.board.check_in_bounds_no_collision(c.position),
            ..*c
        };

        let check_supported = |c: Cube| {
            if self.board.supports(&c.position) || moved_piece.supports(&c.position) {
                c
            } else {
                Cube {
                    error: Some(CubeError::Unsupported),
                    ..c
                }
            }
        };

        return match self.board.check_touches_piece(positions) {
            Ok(cubes) => cubes
                .iter()
                .map(check_inbounds_no_collision)
                .map(check_supported)
                .collect(),
            Err(cubes) => cubes,
        };
    }

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

    pub fn clear_previewed_piece(&mut self) {
        self.previewed_piece = None;
    }

    pub fn calculate_score(&self) -> Score {
        self.board.calculate_score()
    }
}

#[cfg(test)]
mod tests {
    use crate::{game_state::GameState, piece::PieceName, ts_interop::V3, Action};
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
    fn collision() {
        let mut gs = GameState::default();
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));
        gs.apply_action(Action::PlayPreviewedPiece);
        gs.apply_action(Action::SelectPiece(PieceName::OneByTwo));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));
        if let Some(a) = &gs.board_state.previewed_piece {
            println!("{:?}", a);
            assert!(a.iter().any(|c| c.error.is_some()));
        }
    }
}
