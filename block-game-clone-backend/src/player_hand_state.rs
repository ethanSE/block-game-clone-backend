//! contains [PlayerHandState]

use crate::{
    piece::{Piece, PieceName},
    ts_interop::RotationAxis,
};
extern crate alloc;
use alloc::collections::BTreeMap;
use alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

/// represents the pieces in a players hand, their availability, and which (if any) piece is selected by the player
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "pkg/types/PlayerHandState.ts")]
pub struct PlayerHandState {
    /// The piece currently selected by the player, if one is selected
    selected_piece: Option<PieceName>,
    /// The pieces in a player's hand. Pieces already played are represented by an Option::None
    pieces: BTreeMap<PieceName, Option<Piece>>,
}

impl PlayerHandState {
    /// marks the selected piece as unavailable
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

    /// sets a piece as selected by the player
    pub fn set_selected_piece(&mut self, piece_name: PieceName) {
        self.selected_piece = Some(piece_name)
    }

    /// sets which cube among the cubes comprising the selected piece is the representative cube.
    ///
    /// All other cubes are represented as offsets from this new origin.
    pub fn set_selected_piece_origin(&mut self, new_origin: Vector3<f32>) {
        if let Some(selected_piece_name) = self.selected_piece {
            if let Some(Some(piece)) = self.pieces.get_mut(&selected_piece_name) {
                piece.set_origin(new_origin)
            }
        }
    }

    /// Rotates the selected piece PI / 2 about the rotation axis in the positive direction
    pub fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        self.selected_piece.map(|piece_name| {
            self.pieces
                .get_mut(&piece_name)
                .map(|a| a.as_mut().map(|p| p.rotate(rotation_axis)))
        });
    }

    /// returns the selected piece if a piece is selected
    pub fn get_selected_piece(&self) -> Option<Piece> {
        if let Some(piece_name) = self.selected_piece {
            self.pieces[&piece_name].clone()
        } else {
            None
        }
    }

    /// returns all possible rotations of all available pieces in the player's hand
    ///
    /// used when searching for available moves
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
            pieces: BTreeMap::from([
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
