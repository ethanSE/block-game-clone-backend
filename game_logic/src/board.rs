//! Contains [Board]

use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;

use crate::{
    game_mode::{GameMode, TwoPlayerMap},
    player::{self, Player},
};

/// Represents the state of the board
#[derive(Serialize, Deserialize, Debug, TS, Clone)]
#[ts(export, export_to = "pkg/types/Board.ts")]
pub struct Board {
    pub cells: [[[BoardCell; 8]; 8]; 8],
    /// used to construct, show available space to player
    #[ts(type = "Array<Array<number>>")]
    pub height_limits: Vec<Vec<usize>>,
    /// useful for centering a camera
    #[ts(type = "[number, number, number]")]
    pub center: Vector3<f32>,
}

fn v3_to_index(v3: Vector3<f32>) -> Option<(usize, usize, usize)> {
    if v3.x >= 0.0 && v3.y >= 0.0 && v3.z >= 0.0 {
        Some((v3.x as usize, v3.y as usize, v3.z as usize))
    } else {
        None
    }
}

impl Board {
    fn map_to_heights(map: TwoPlayerMap) -> Vec<Vec<usize>> {
        match map {
            TwoPlayerMap::Tower => vec![
                vec![4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4],
                vec![4, 4, 4, 4, 4],
            ],
            TwoPlayerMap::Pyramid => vec![
                vec![1, 1, 1, 1, 1, 1, 1, 1],
                vec![1, 2, 2, 2, 2, 2, 2, 1],
                vec![1, 2, 3, 3, 3, 3, 2, 1],
                vec![1, 2, 3, 4, 4, 3, 2, 1],
                vec![1, 2, 3, 4, 4, 3, 2, 1],
                vec![1, 2, 3, 3, 3, 3, 2, 1],
                vec![1, 2, 2, 2, 2, 2, 2, 1],
                vec![1, 1, 1, 1, 1, 1, 1, 1],
            ],
            TwoPlayerMap::Stairs => vec![
                vec![1, 2, 3, 4, 5, 5, 5, 5],
                vec![1, 2, 3, 4, 5, 5, 5, 5],
                vec![1, 2, 3, 4, 5, 5, 5, 5],
                vec![1, 2, 3, 4, 5, 5, 5, 5],
            ],
            TwoPlayerMap::Wall => vec![
                vec![2, 2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2, 2],
                vec![2, 2, 2, 2, 2, 2],
                vec![0, 0, 0, 2, 2, 2],
                vec![0, 0, 0, 2, 2, 2],
                vec![0, 0, 0, 2, 2, 2],
                vec![0, 0, 0, 2, 2, 2],
                vec![0, 0, 0, 2, 2, 2],
            ],
        }
    }

    pub fn new(game_mode: GameMode) -> Self {
        Self::new_board_from_2d_heights(match game_mode {
            GameMode::Solitaire(_) => todo!(),
            GameMode::TwoPlayer(map) => Self::map_to_heights(map),
            GameMode::VSGreedyAI(map) => Self::map_to_heights(map),
        })
    }

    pub fn calculate_score(&self) -> HashMap<Player, i8> {
        let highests: Vec<Player> = self
            .height_limits
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(|(z, y_max)| {
                        self.get_highest_player(x as i8, z as i8, *y_max as i8)
                    })
                    .collect::<Vec<_>>()
            })
            .collect();

        let total = highests.len();

        let p1s = highests
            .into_iter()
            .filter(|item| item == &Player::P1)
            .count();

        HashMap::from([(Player::P1, p1s as i8), (Player::P2, (total - p1s) as i8)])
    }

    /// returns all available positions on the board. Used for searching for possible moves
    pub fn get_available_positions(&self) -> Vec<Vector3<f32>> {
        self.height_limits
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .flat_map(|(z, y_max)| {
                        (0..=*y_max).filter_map(move |y| match self.get_from_index((x, y, z)) {
                            Some(&BoardCell::Empty) => {
                                Some(Vector3::new(x as f32, y as f32, z as f32))
                            }
                            _ => None,
                        })
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    /// Returns the highest player for a given column, used for scoring
    fn get_highest_player(&self, x: i8, z: i8, y_max: i8) -> Option<Player> {
        // for each pair (x,z) starting from height y and working downwards, find the first board cell that is owned by a player
        (0..y_max).rev().find_map(
            |y| match self.get(Vector3::new(x as f32, y as f32, z as f32)) {
                Some(&BoardCell::Player(player)) => Some(player),
                _ => None,
            },
        )
    }

    fn get_from_index(&self, (x, y, z): (usize, usize, usize)) -> Option<&BoardCell> {
        self.cells
            .get(x)
            .and_then(|b| b.get(y))
            .and_then(|c| c.get(z))
    }

    fn get_from_index_mut(&mut self, (x, y, z): (usize, usize, usize)) -> Option<&mut BoardCell> {
        self.cells
            .get_mut(x)
            .and_then(|b| b.get_mut(y))
            .and_then(|c| c.get_mut(z))
    }

    pub fn get(&self, index: Vector3<f32>) -> Option<&BoardCell> {
        if let Some((x, y, z)) = v3_to_index(index) {
            self.get_from_index((x, y, z))
        } else {
            None
        }
    }

    pub fn get_mut(&mut self, index: Vector3<f32>) -> Option<&mut BoardCell> {
        if let Some((x, y, z)) = v3_to_index(index) {
            self.get_from_index_mut((x, y, z))
        } else {
            None
        }
    }

    pub fn add_cubes(&mut self, cubes: &Vec<Cube>) {
        for cube in cubes {
            if let Some(bc) = self.get_mut(cube.position) {
                *bc = BoardCell::Player(cube.player)
            }
        }
    }

    pub fn supports(&self, position: &Vector3<f32>) -> bool {
        let supported_by_piece = self
            .get(*position - Vector3::<f32>::new(0.0, 1.0, 0.0))
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

        positions.iter().filter_map(|p| self.get(*p)).collect()
    }

    /// implements game rule requiring players to play touching their own piece.
    ///
    /// The first player may play anywhere. The second must touch the first
    pub fn check_touches_piece(&self, new_piece_coords: Vec<Cube>) -> Result<Vec<Cube>, Vec<Cube>> {
        let player = new_piece_coords[0].player;

        // the first player can play anywhere, the second player mst touch the first, after this, all players must touch their own piece
        let player_to_check_for = self
            .player_has_played(player)
            .or(self.player_has_played(player.get_other()));

        if let Some(player) = player_to_check_for {
            if new_piece_coords
                .iter()
                .flat_map(|p| self.get_adjacent_cells(&p.position))
                .any(|bc| *bc == BoardCell::Player(player))
            {
                Ok(new_piece_coords)
            } else {
                //embed error in all cubes
                Err(new_piece_coords
                    .iter()
                    .map(|cube| Cube {
                        error: Some(CubeError::NotTouchingPiece),
                        ..*cube
                    })
                    .collect())
            }
        } else {
            // neither player has played, free to play anywhere
            Ok(new_piece_coords)
        }
    }

    /// determines whether or not a player has played
    fn player_has_played(&self, player: Player) -> Option<Player> {
        self.cells
            .into_iter()
            .flat_map(|a| a.into_iter().flat_map(|b| b.into_iter()))
            .find(|bc| bc == &BoardCell::Player(player))
            .and(Some(player))
    }

    pub fn check_in_bounds_no_collision(&self, position: Vector3<f32>) -> Option<CubeError> {
        match self.get(position) {
            Some(BoardCell::Empty) => None,
            Some(BoardCell::Player(_)) => Some(CubeError::Collision),
            _ => Some(CubeError::OutOfBounds),
        }
    }

    fn new_board_from_2d_heights(heights_2d: Vec<Vec<usize>>) -> Self {
        let mut board = [[[BoardCell::OutOfBounds; 8]; 8]; 8];
        for (ix, x) in heights_2d.iter().enumerate() {
            for (iz, ymax) in x.iter().enumerate() {
                for y in 0..*ymax {
                    board[ix][y][iz] = BoardCell::Empty
                }
            }
        }

        let mid_x = (heights_2d.len() as f32 - 1.0) / 2.0;

        let mid_y = (*heights_2d
            .iter()
            .flatten()
            .max_by(|a, b| a.partial_cmp(b).unwrap())
            .unwrap() as f32
            - 1.0)
            / 2.0;

        let mid_z = (heights_2d
            .iter()
            .max_by(|a, b| a.len().cmp(&b.len()))
            .unwrap()
            .len() as f32
            - 1.0)
            / 2.0;

        Self {
            cells: board,
            height_limits: heights_2d,
            center: Vector3::<f32>::new(mid_x, mid_y, mid_z),
        }
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new_board_from_2d_heights(Self::map_to_heights(TwoPlayerMap::Tower))
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Vector3;

    use crate::{action::V3, board_state::BoardState, game_state::GameState, player::Player};

    #[test]
    fn print() {
        let mut gs = GameState::default();

        gs.apply_action(crate::action::Action::SelectPiece(
            crate::piece::PieceName::Corner,
        ));

        gs.apply_action(crate::action::Action::PreviewPiece(V3(
            Vector3::<f32>::new(0.0, 0.0, 0.0),
        )));

        gs.apply_action(crate::action::Action::PlayPreviewedPiece);

        // assert!(gs.score[&Player::P1] == 3);
        // assert!(gs.score[&Player::P2] == 0);

        println!("{}", serde_json::to_string(&gs).unwrap())
    }

    #[test]
    fn score() {
        let b = BoardState::new(crate::game_mode::GameMode::TwoPlayer(
            crate::game_mode::TwoPlayerMap::Stairs,
        ));

        let score = b.calculate_score();
        println!("{:?}", score);
        assert!(score.contains_key(&Player::P1));
        assert!(score.contains_key(&Player::P2));
    }
}

#[derive(Default, Serialize, Deserialize, Debug, PartialEq, TS, Clone, Copy)]
#[ts(export, export_to = "pkg/types/BoardCell.ts")]
#[serde(tag = "type", content = "data")]
pub enum BoardCell {
    #[default]
    Empty,
    Player(Player),
    OutOfBounds,
}

#[derive(Serialize, Deserialize, Debug, Clone, Copy, TS, PartialEq)]
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
