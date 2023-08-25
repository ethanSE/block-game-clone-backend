use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::ops::{Index, IndexMut};
use ts_rs::TS;

use crate::{
    piece::{Piece, PieceName},
    player::Player,
    player_hand_state::PlayerHandState,
    ts_interop::RotationAxis,
};

#[derive(Serialize, Deserialize, Default, TS, Clone, Debug)]
#[ts(export, export_to = "pkg/types/PlayerState.ts")]
pub struct PlayerState {
    pub current_player: Player,
    pub players: Players,
}

#[derive(Serialize, Deserialize, Default, TS, Clone, Debug)]
#[ts(export, export_to = "pkg/types/Players.ts")]
pub struct Players {
    p1: PlayerHandState,
    p2: PlayerHandState,
}

impl PlayerState {
    pub fn toggle_current_player(&mut self) {
        self.players[self.current_player].clear_selected_piece();
        self.current_player = self.current_player.get_other();
    }

    pub fn select_piece(&mut self, piece_name: PieceName) {
        self.players[self.current_player].set_selected_piece(piece_name)
    }

    pub fn clear_selected_piece(&mut self) {
        self.players[self.current_player].clear_selected_piece();
    }

    pub fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        self.players[self.current_player].rotate_selected_piece(rotation_axis)
    }

    pub fn set_selected_piece_origin(&mut self, new_origin: Vector3<f32>) {
        self.players[self.current_player].set_selected_piece_origin(new_origin)
    }

    pub fn play_selected_piece(&mut self) {
        self.players[self.current_player].play_selected_piece();
        self.toggle_current_player();
    }

    pub fn get_selected_piece(&mut self) -> Option<(Player, Piece)> {
        if let Some(piece) = self.players[self.current_player].get_selected_piece() {
            Some((self.current_player, piece.clone()))
        } else {
            None
        }
    }

    pub fn get_available_piece_rotations(&self) -> Vec<(PieceName, Piece)> {
        self.players.clone()[self.current_player].get_available_piece_rotations()
    }
}

impl Index<Player> for Players {
    type Output = PlayerHandState;
    fn index(&self, index: Player) -> &Self::Output {
        match index {
            Player::P1 => &self.p1,
            Player::P2 => &self.p2,
        }
    }
}

impl IndexMut<Player> for Players {
    fn index_mut(&mut self, index: Player) -> &mut Self::Output {
        match index {
            Player::P1 => &mut self.p1,
            Player::P2 => &mut self.p2,
        }
    }
}