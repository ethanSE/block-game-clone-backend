use crate::{
    piece::{Piece, PieceName},
    ts_interop::RotationAxis,
};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "pkg/types/PlayerHandState.ts")]
pub struct PlayerHandState {
    selected_piece: Option<PieceName>,
    pieces: HashMap<PieceName, Option<Piece>>,
}

impl PlayerHandState {
    pub fn play_selected_piece(&mut self) {
        self.selected_piece.map(|selected_piece_name| {
            self.pieces
                .get_mut(&selected_piece_name)
                .map(|entry| *entry = None)
        });
    }

    pub fn clear_selected_piece(&mut self) {
        self.selected_piece = None
    }

    pub fn set_selected_piece(&mut self, piece_name: PieceName) {
        self.selected_piece = Some(piece_name)
    }

    pub fn set_selected_piece_origin(&mut self, new_origin: Vector3<f32>) {
        if let Some(selected_piece_name) = self.selected_piece {
            if let Some(Some(piece)) = self.pieces.get_mut(&selected_piece_name) {
                piece.set_origin(new_origin)
            }
        }
    }

    pub fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        self.selected_piece.map(|piece_name| {
            self.pieces
                .get_mut(&piece_name)
                .map(|a| a.as_mut().map(|p| p.rotate(rotation_axis)))
        });
    }

    pub fn get_selected_piece(&self) -> Option<Piece> {
        if let Some(piece_name) = self.selected_piece {
            self.pieces[&piece_name].clone()
        } else {
            None
        }
    }

    pub fn get_available_piece_rotations(&self) -> Vec<(PieceName, Piece)> {
        let available_pieces = self
            .pieces
            .iter()
            .filter_map(|(name, piece)| piece.as_ref().map(|piece| (name, piece)));

        available_pieces
            .flat_map(|(name, piece)| {
                piece
                    .get_available_piece_rotations()
                    .iter()
                    .map(|piece| (*name, piece.clone()))
                    .collect::<Vec<_>>()
            })
            .collect()
    }
}

impl Default for PlayerHandState {
    fn default() -> Self {
        Self {
            selected_piece: Default::default(),
            pieces: HashMap::from([
                (
                    PieceName::OneByTwo,
                    Some(Piece::from_vec_i8_array(vec![[0, 0, 0], [0, 0, 1]])),
                ),
                (
                    PieceName::OneByThree,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 0, 2],
                    ])),
                ),
                (
                    PieceName::OneByFour,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 0, 2],
                        [0, 0, 3],
                    ])),
                ),
                (
                    PieceName::TwoByTwo,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 0],
                        [0, 1, 1],
                    ])),
                ),
                (
                    PieceName::Z,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 1],
                        [0, 1, 2],
                    ])),
                ),
                (
                    PieceName::T,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 1],
                        [0, 0, 2],
                    ])),
                ),
                (
                    PieceName::L,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 0, 2],
                        [0, 1, 2],
                    ])),
                ),
                (
                    PieceName::ShortL,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 1],
                    ])),
                ),
                (
                    PieceName::RightScrew,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 1],
                        [1, 1, 1],
                    ])),
                ),
                (
                    PieceName::LeftScrew,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 1],
                        [-1, 1, 1],
                    ])),
                ),
                (
                    PieceName::Corner,
                    Some(Piece::from_vec_i8_array(vec![
                        [0, 0, 0],
                        [0, 0, 1],
                        [0, 1, 1],
                        [1, 0, 1],
                    ])),
                ),
            ]),
        }
    }
}
