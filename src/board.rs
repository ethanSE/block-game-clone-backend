use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::{
    game_mode::{GameMode, TwoPlayerMap},
    player::{self, Player},
    ts_interop::Score,
};

#[derive(Serialize, Deserialize, Debug, TS)]
#[ts(export, export_to = "pkg/types/Board.ts")]
pub struct Board {
    pub cells: [[[BoardCell; 8]; 8]; 8],
    #[ts(type = "Array<Array<number>>")]
    pub height_limits: Vec<Vec<usize>>,
    #[ts(type = "[number, number, number]")]
    pub center: Vector3<f32>,
}

impl Board {
    pub fn new(game_mode: GameMode) -> Self {
        match game_mode {
            GameMode::Solitaire(_) => todo!(),
            GameMode::TwoPlayer(TwoPlayerMap::Tower) => Board::new_tower(),
            GameMode::TwoPlayer(TwoPlayerMap::Pyramid) => Board::new_pyramid(),
            GameMode::TwoPlayer(TwoPlayerMap::Stairs) => Board::new_corner(),
            GameMode::TwoPlayer(TwoPlayerMap::Wall) => Board::new_wall(),
        }
    }

    pub fn calculate_score(&self) -> Score {
        let highests: Vec<Vec<_>> = self
            .height_limits
            .iter()
            .enumerate()
            .map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .map(|(z, y_max)| self.get_highest_player(x as i8, z as i8, *y_max as i8))
                    .collect()
            })
            .collect();

        let column_top_players = highests
            .into_iter()
            .flat_map(|a| a.into_iter())
            .flatten()
            .collect::<Vec<_>>();

        let p1s = column_top_players
            .iter()
            .filter(|item| item == &&Player::P1)
            .count();

        let p2s = column_top_players.len() - p1s;

        Score { p1: p1s, p2: p2s }
    }

    fn get_highest_player(&self, x: i8, z: i8, y_max: i8) -> Option<Player> {
        // for each pair (x,z) starting from height y and working downwards, find the first board cell that is owned by a player
        (0..y_max)
            .rev()
            .find_map(|y| match self.get(Vector3::new(x, y, z)) {
                Some(&BoardCell::Player(player)) => Some(player),
                _ => None,
            })
    }

    fn get_mut(&mut self, index: Vector3<i8>) -> Option<&mut BoardCell> {
        let x: usize = index.x.try_into().ok()?;
        let y: usize = index.y.try_into().ok()?;
        let z: usize = index.z.try_into().ok()?;

        if let Some(bc) = self
            .cells
            .get_mut(x)
            .and_then(|b| b.get_mut(y))
            .and_then(|c| c.get_mut(z))
        {
            Some(bc)
        } else {
            None
        }
    }

    fn get(&self, index: Vector3<i8>) -> Option<&BoardCell> {
        let x: usize = index.x.try_into().ok()?;
        let y: usize = index.y.try_into().ok()?;
        let z: usize = index.z.try_into().ok()?;

        if let Some(bc) = self
            .cells
            .get(x)
            .and_then(|b| b.get(y))
            .and_then(|c| c.get(z))
        {
            Some(bc)
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

    pub fn supports(&self, position: &Vector3<i8>) -> bool {
        let supported_by_piece = self
            .get(*position - Vector3::<i8>::new(0, 1, 0))
            .is_some_and(|bc| *bc != BoardCell::Empty);

        let is_on_ground = position.y == 0;

        supported_by_piece || is_on_ground
    }

    fn get_adjacent_cells(&self, position: &Vector3<i8>) -> Vec<&BoardCell> {
        let positions = vec![
            position - Vector3::<i8>::new(1, 0, 0),
            position - Vector3::<i8>::new(-1, 0, 0),
            position - Vector3::<i8>::new(0, 1, 0),
            position - Vector3::<i8>::new(0, -1, 0),
            position - Vector3::<i8>::new(0, 0, 1),
            position - Vector3::<i8>::new(0, 0, -1),
        ];

        positions.iter().filter_map(|p| self.get(*p)).collect()
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
        let mut all_indices = self
            .cells
            .iter()
            .flat_map(|a| a.iter().flat_map(|b| b.iter()));
        all_indices
            .find(|bc| **bc == BoardCell::Player(player))
            .and(Some(player))
    }

    pub fn check_in_bounds_no_collision(&self, position: Vector3<i8>) -> Option<CubeError> {
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

    fn new_pyramid() -> Self {
        let heights = vec![
            vec![1, 1, 1, 1, 1, 1, 1, 1],
            vec![1, 2, 2, 2, 2, 2, 2, 1],
            vec![1, 2, 3, 3, 3, 3, 2, 1],
            vec![1, 2, 3, 4, 4, 3, 2, 1],
            vec![1, 2, 3, 4, 4, 3, 2, 1],
            vec![1, 2, 3, 3, 3, 3, 2, 1],
            vec![1, 2, 2, 2, 2, 2, 2, 1],
            vec![1, 1, 1, 1, 1, 1, 1, 1],
        ];
        Self::new_board_from_2d_heights(heights)
    }

    fn new_tower() -> Self {
        let heights = vec![
            vec![4, 4, 4, 4, 4],
            vec![4, 4, 4, 4, 4],
            vec![4, 4, 4, 4, 4],
            vec![4, 4, 4, 4, 4],
        ];
        Self::new_board_from_2d_heights(heights)
    }

    fn new_corner() -> Self {
        let heights = vec![
            vec![1, 2, 3, 4, 5, 5, 5, 5],
            vec![1, 2, 3, 4, 5, 5, 5, 5],
            vec![1, 2, 3, 4, 5, 5, 5, 5],
            vec![1, 2, 3, 4, 5, 5, 5, 5],
        ];
        Self::new_board_from_2d_heights(heights)
    }

    fn new_wall() -> Self {
        let heights = vec![
            vec![2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2],
            vec![2, 2, 2, 2, 2, 2],
            vec![0, 0, 0, 2, 2, 2],
            vec![0, 0, 0, 2, 2, 2],
            vec![0, 0, 0, 2, 2, 2],
            vec![0, 0, 0, 2, 2, 2],
            vec![0, 0, 0, 2, 2, 2],
        ];
        Self::new_board_from_2d_heights(heights)
    }
}

impl Default for Board {
    fn default() -> Self {
        Self::new_tower()
    }
}

#[cfg(test)]
mod test {
    use nalgebra::Vector3;

    use crate::{game_state::GameState, ts_interop::V3};

    #[test]
    fn print() {
        let mut gs = GameState::default();

        gs.apply_action(crate::ts_interop::Action::SelectPiece(
            crate::piece::PieceName::Corner,
        ));

        gs.apply_action(crate::ts_interop::Action::PreviewPiece(V3(
            Vector3::<i8>::new(0, 0, 0),
        )));

        gs.apply_action(crate::ts_interop::Action::PlayPreviewedPiece);

        assert!(gs.score.p1 == 3);
        assert!(gs.score.p2 == 0);

        println!("{}", serde_json::to_string(&gs).unwrap())
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
    pub position: Vector3<i8>,
    pub error: Option<CubeError>,
}
