use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    piece::round_vec3,
    player::{self, Player},
};

#[derive(Serialize, Deserialize, Default, Debug, TS)]
#[ts(export, export_to = "pkg/types/Board.ts")]
pub struct Board(pub [[[BoardCell; 4]; 4]; 5]);

impl Board {
    pub fn update_at(&mut self, index: Vector3<f32>, value: BoardCell) {
        if let Some(bc) = self
            .0
            .get_mut(index.x as usize)
            .and_then(|b| b.get_mut(index.y as usize))
            .and_then(|c| c.get_mut(index.z as usize))
        {
            *bc = value
        }
    }

    pub fn get_at(&self, index: &Vector3<f32>) -> Option<&BoardCell> {
        self.0
            .get(index.x as usize)
            .and_then(|b| b.get(index.y as usize))
            .and_then(|c| c.get(index.z as usize))
    }

    pub fn supports(&self, position: &Vector3<f32>) -> bool {
        let supported_by_piece = self
            .get_at(&(*position - Vector3::<f32>::new(0.0, 1.0, 0.0)))
            .is_some_and(|bc| *bc != BoardCell::Empty);

        let is_on_ground = position.y == 0.0;

        supported_by_piece || is_on_ground
    }

    fn get_adjacent_cells(&self, position: &Vector3<f32>) -> Vec<&BoardCell> {
        let positions = vec![
            position - Vector3::<f32>::new(1.0, 0.0, 0.0),
            position - Vector3::<f32>::new(-1.0, 0.0, 0.0),
            position - Vector3::<f32>::new(0.0, 1.0, 0.0),
            position - Vector3::<f32>::new(0.0, -1.0, 0.0),
            position - Vector3::<f32>::new(0.0, 0.0, 1.0),
            position - Vector3::<f32>::new(0.0, 0.0, -1.0),
        ];

        positions
            .iter()
            .filter_map(|p| self.get_at(&round_vec3(*p)))
            .collect()
    }

    pub fn check_touches_piece(&self, mut coords: Vec<Cube>) -> Result<Vec<Cube>, Vec<Cube>> {
        let player = coords[0].player;

        let player_to_check_for = self
            .player_has_played(player)
            .or(self.player_has_played(player.get_other()));

        if let Some(player) = player_to_check_for {
            if coords
                .iter()
                .flat_map(|p| self.get_adjacent_cells(&p.position))
                .any(|bc| *bc == BoardCell::Player(player))
            {
                Ok(coords)
            } else {
                for c in &mut coords {
                    c.error = Some(CubeError::NotTouchingPiece)
                }
                Err(coords)
            }
        } else {
            Ok(coords)
        }
    }

    fn player_has_played(&self, player: Player) -> Option<Player> {
        let mut all_indices = self.0.iter().flat_map(|a| a.iter().flat_map(|b| b.iter()));
        all_indices
            .find(|bc| **bc == BoardCell::Player(player))
            .and(Some(player))
    }

    pub fn check_in_bounds_no_collision(&self, vc: &Cube) -> Cube {
        match self.get_at(&vc.position) {
            Some(BoardCell::Empty) => *vc,
            Some(BoardCell::Player(_)) => Cube {
                player: vc.player,
                position: vc.position,
                error: Some(CubeError::Collision),
            },
            None => Cube {
                player: vc.player,
                position: vc.position,
                error: Some(CubeError::OutOfBounds),
            },
        }
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, TS)]
#[ts(export, export_to = "pkg/types/BoardCell.ts")]
#[serde(tag = "type", content = "data")]
pub enum BoardCell {
    #[default]
    Empty,
    Player(Player),
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, TS)]
#[ts(export, export_to = "pkg/types/CubeError.ts")]
pub enum CubeError {
    Collision,
    Unsupported,
    OutOfBounds,
    NotTouchingPiece,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, TS)]
#[ts(export, export_to = "pkg/types/Cube.ts")]
pub struct Cube {
    pub player: player::Player,
    #[ts(type = "[number,number,number]")]
    pub position: Vector3<f32>,
    pub error: Option<CubeError>,
}
