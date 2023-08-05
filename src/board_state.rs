use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    board::{Board, BoardCell, Cube, CubeError},
    piece::Piece,
    player::Player,
};

#[derive(Serialize, Deserialize, Default, Debug, TS)]
#[ts(export, export_to = "pkg/types/BoardState.ts")]
pub struct BoardState {
    pub pieces: Board,
    pub previewed_piece: Option<Vec<Cube>>,
}

impl BoardState {
    pub fn preview_piece(&mut self, current_player: Player, piece: &Piece, position: Vector3<f32>) {
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

        let check_supported = |vc: Cube| {
            if self.pieces.supports(&vc.position) || moved_piece.supports(&vc.position) {
                vc
            } else {
                Cube {
                    error: Some(CubeError::Unsupported),
                    ..vc
                }
            }
        };

        let check_inbounds_no_collision = |&c| self.pieces.check_in_bounds_no_collision(&c);

        self.previewed_piece = Some(match self.pieces.check_touches_piece(positions) {
            Ok(cubes) => cubes
                .iter()
                .map(check_inbounds_no_collision)
                .map(check_supported)
                .collect(),
            Err(cubes) => cubes,
        });
    }

    pub fn play_selected_piece(&mut self) -> Result<(), ()> {
        if let Some(preview_cubes) = &mut self.previewed_piece {
            if preview_cubes.iter().all(|c| c.error.is_none()) {
                for piece in preview_cubes {
                    self.pieces
                        .update_at(piece.position, BoardCell::Player(piece.player))
                }
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
}

#[cfg(test)]
mod tests {
    use crate::{board_state::BoardState, game_state::GameState, piece::PieceName, Action, V3};
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
}
