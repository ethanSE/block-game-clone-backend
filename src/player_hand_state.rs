use crate::{
    hand::Hand,
    piece::{Piece, PieceName},
    RotationAxis,
};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Default, TS)]
#[ts(export, export_to = "pkg/types/PlayerHandState.ts")]
pub struct PlayerHandState {
    selected_piece: Option<PieceName>,
    pieces: Hand,
}

impl PlayerHandState {
    pub fn play_selected_piece(&mut self) {
        if let Some(selected_piece_name) = self.selected_piece {
            *self.pieces.get_mut(selected_piece_name) = None;
        }
    }

    pub fn clear_selected_piece(&mut self) {
        self.selected_piece = None
    }

    pub fn set_selected_piece(&mut self, piece_name: PieceName) {
        self.selected_piece = Some(piece_name)
    }

    pub fn set_selected_piece_origin(&mut self, new_origin: Vector3<i8>) {
        if let Some(selected_piece_name) = self.selected_piece {
            if let Some(a) = self.pieces.get_mut(selected_piece_name) {
                a.set_origin(new_origin)
            }
        }
    }

    pub fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        if let Some(selected_piece_name) = self.selected_piece {
            if let Some(a) = self.pieces.get_mut(selected_piece_name) {
                a.rotate(rotation_axis)
            }
        }
    }

    pub fn get_selected_piece(&mut self) -> &Option<Piece> {
        if let Some(piece_name) = self.selected_piece {
            self.pieces.get_mut(piece_name)
        } else {
            &None
        }
    }
}
