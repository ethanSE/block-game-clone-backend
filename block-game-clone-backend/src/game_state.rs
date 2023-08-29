use itertools::Itertools;
use nalgebra::Vector3;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, ops::Index};
use ts_rs::TS;

use crate::{
    board_state::BoardState,
    game_mode::GameMode,
    piece::PieceName,
    player::Player,
    player_state::PlayerState,
    ts_interop::{Action, RotationAxis, V3},
};

#[derive(Serialize, Deserialize, TS, Clone, Debug, Default)]
#[ts(export, export_to = "pkg/types/GameState.ts")]
pub struct GameState {
    pub player_state: PlayerState,
    pub board_state: BoardState,
    pub game_mode: GameMode,
    pub score: HashMap<Player, i8>,
    pub game_ended: bool,
}

impl GameState {
    pub(crate) fn new(game_mode: GameMode) -> Self {
        Self {
            game_mode,
            player_state: PlayerState::new(game_mode),
            board_state: BoardState::new(game_mode),
            score: match game_mode {
                GameMode::Solitaire(_) => HashMap::from([(Player::P1, 0)]),
                _ => HashMap::from([(Player::P1, 0), (Player::P2, 0)]),
            },
            game_ended: false,
        }
    }

    pub fn apply_action(&mut self, action: Action) {
        match action {
            Action::SelectPiece(name) => self.select_piece(name),
            Action::ClearSelectedPiece => self.clear_selected_piece(),
            Action::SetSelectedPieceOrigin(V3(new_origin)) => {
                self.set_selected_piece_origin(new_origin)
            }
            Action::RotateSelectedPiece(rotation_axis) => self.rotate_selected_piece(rotation_axis),
            Action::PreviewPiece(V3(position)) => self.preview_piece(position),
            Action::PlayPreviewedPiece => {
                let _ = self.play_previewed_piece();
            }
            Action::PassTurn => self.pass_turn(),
            Action::Reset => self.reset(),
            Action::MakeGreedyAIMove => self.make_greedy_ai_move(),
        }
    }

    fn select_piece(&mut self, piece_name: PieceName) {
        self.player_state.select_piece(piece_name);
        self.board_state.clear_previewed_piece()
    }

    fn clear_selected_piece(&mut self) {
        self.board_state.clear_previewed_piece();
        self.player_state.clear_selected_piece();
    }

    fn rotate_selected_piece(&mut self, rotation_axis: RotationAxis) {
        self.player_state.rotate_selected_piece(rotation_axis);
        self.board_state.clear_previewed_piece()
    }

    fn set_selected_piece_origin(&mut self, new_origin: Vector3<f32>) {
        self.player_state.set_selected_piece_origin(new_origin)
    }

    fn preview_piece(&mut self, position: Vector3<f32>) {
        if let Some((current_player, piece)) = self.player_state.get_selected_piece() {
            self.board_state
                .preview_piece(current_player, piece, position)
        }
    }

    fn play_previewed_piece(&mut self) -> Result<(), ()> {
        if let Ok(()) = self.board_state.play_selected_piece() {
            self.player_state.play_selected_piece();
            self.score = self.board_state.calculate_score();
            Ok(())
        } else {
            Err(())
        }
    }

    fn pass_turn(&mut self) {
        println!("turn passed");
        self.board_state.clear_previewed_piece();
        self.player_state.toggle_current_player();
        self.determine_game_ended();
    }

    fn reset(&mut self) {
        self.player_state = PlayerState::new(self.game_mode);
        self.board_state = BoardState::default();
    }

    fn determine_game_ended(&mut self) {
        if !self.available_move_exists(Player::P1) && !self.available_move_exists(Player::P2) {
            self.game_ended = true;
        }
    }

    fn available_move_exists(&self, player: Player) -> bool {
        self.player_state
            .players
            .index(&player)
            .get_available_piece_rotations()
            .into_iter()
            .cartesian_product(self.board_state.board.get_available_positions().into_iter())
            .filter_map(|((piece_name, piece), position)| {
                let mut next_game_state = self.clone();
                next_game_state.apply_action(Action::SelectPiece(piece_name));
                next_game_state.board_state.preview_piece(
                    self.player_state.current_player,
                    piece,
                    position,
                );
                next_game_state.play_previewed_piece().ok()
            })
            .next()
            .is_some()
    }

    pub fn make_greedy_ai_move(&mut self) {
        let all_moves = self
            .player_state
            .get_available_piece_rotations()
            .into_iter()
            .cartesian_product(self.board_state.board.get_available_positions().into_iter());

        let best_move_resulting_gs = all_moves
            .filter_map(|((piece_name, piece), position)| {
                let mut next_game_state = self.clone();
                next_game_state.apply_action(Action::SelectPiece(piece_name));
                next_game_state.board_state.preview_piece(
                    self.player_state.current_player,
                    piece,
                    position,
                );
                match next_game_state.play_previewed_piece() {
                    Ok(_) => Some((
                        next_game_state.score[&self.player_state.current_player]
                            - next_game_state.score[&self.player_state.current_player.get_other()],
                        next_game_state,
                    )),
                    Err(_) => None,
                }
            })
            .max_by(|a, b| a.0.cmp(&b.0));

        match best_move_resulting_gs {
            Some((_score, next_gs)) => *self = next_gs,
            None => self.apply_action(Action::PassTurn),
        }
    }
}

#[cfg(test)]
mod tests {
    use nalgebra::Vector3;

    use crate::{
        game_mode::GameMode,
        game_state::GameState,
        piece::PieceName,
        ts_interop::{Action, V3},
    };

    #[test]
    fn test_multiple_actions() {
        let mut gs = GameState::new(GameMode::default());

        gs.apply_action(Action::SelectPiece(PieceName::Corner));
        gs.apply_action(Action::PreviewPiece(V3(Vector3::<f32>::new(0.0, 0.0, 0.0))));
        gs.apply_action(Action::PlayPreviewedPiece);

        let gs_str = serde_json::to_string(&gs).unwrap();
        println!("{}", gs_str)
    }

    #[test]
    fn test_ai_move() {
        let mut gs = GameState::new(GameMode::VSGreedyAI(
            crate::game_mode::TwoPlayerMap::Pyramid,
        ));

        gs.apply_action(Action::MakeGreedyAIMove);
        println!("{:?}", gs);
        gs.apply_action(Action::MakeGreedyAIMove);
        println!("{:?}", gs);
    }

    #[test]
    fn test_ai_vs_ai() {
        let mut gs = GameState::new(GameMode::VSGreedyAI(
            crate::game_mode::TwoPlayerMap::Pyramid,
        ));

        while !gs.game_ended {
            println!("{:?}", gs);
            gs.make_greedy_ai_move();
            println!("made move");
        }
        println!("{:?}", gs);
    }
}
