//! manages information about players

use crate::{
    game_mode::GameMode,
    piece::{Piece, PieceName},
    player::Player,
    player_hand_state::PlayerHandState,
    ts_interop::RotationAxis,
};
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

/// manages information about players
///
/// who is the current player, pieces in players' hands
#[derive(Serialize, Deserialize, TS, Clone, Debug)]
#[ts(export, export_to = "pkg/types/PlayerState.ts")]
pub(crate) struct PlayerState {
    pub(crate) current_player: Player,
    pub(crate) players: HashMap<Player, PlayerHandState>,
}

impl Default for PlayerState {
    fn default() -> Self {
        PlayerState::new(GameMode::default())
    }
}

impl PlayerState {
    pub fn new(game_mode: GameMode) -> Self {
        Self {
            current_player: Player::default(),
            players: match game_mode {
                GameMode::Solitaire(_) => HashMap::from([(Player::P1, PlayerHandState::default())]),
                _ => HashMap::from([
                    (Player::P1, PlayerHandState::default()),
                    (Player::P2, PlayerHandState::default()),
                ]),
            },
        }
    }

    pub fn toggle_current_player(&mut self) {
        if let Some(p) = self.players.get_mut(&self.current_player) {
            p.clear_selected_piece()
        }
        self.current_player = self.current_player.get_other();
    }

    pub fn select_piece(&mut self, piece_name: PieceName) {
        if let Some(p) = self.players.get_mut(&self.current_player) {
            p.set_selected_piece(piece_name)
        }
    }

    pub fn clear_selected_piece(&mut self) {
        if let Some(p) = self.players.get_mut(&self.current_player) {
            p.clear_selected_piece()
        }
    }

    pub fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        if let Some(p) = self.players.get_mut(&self.current_player) {
            p.rotate_selected_piece(rotation_axis)
        }
    }

    pub fn set_selected_piece_origin(&mut self, new_origin: Vector3<f32>) {
        if let Some(p) = self.players.get_mut(&self.current_player) {
            p.set_selected_piece_origin(new_origin)
        }
    }

    pub fn play_selected_piece(&mut self) {
        if let Some(p) = self.players.get_mut(&self.current_player) {
            p.play_selected_piece()
        }
        self.toggle_current_player();
    }

    pub fn get_selected_piece(&mut self) -> Option<(Player, Piece)> {
        match self
            .players
            .get(&self.current_player)
            .map(|p| p.get_selected_piece())
        {
            Some(Some(piece)) => Some((self.current_player, piece)),
            _ => None,
        }
    }

    pub fn get_available_piece_rotations(&self) -> Vec<(PieceName, Piece)> {
        self.players.clone()[&self.current_player].get_available_piece_rotations()
    }
}
