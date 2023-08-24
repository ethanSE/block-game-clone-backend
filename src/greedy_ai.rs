use itertools::Itertools;

use crate::{game_state::GameState, ts_interop::Action};

pub fn greedy_ai(gs: GameState) -> GameState {
    let all_moves = gs
        .get_available_piece_rotations()
        .into_iter()
        .cartesian_product(gs.board_state.board.get_available_positions().into_iter());

    let best_move_gs = all_moves
        .filter_map(|((piece_name, piece), position)| {
            let mut next_game_state = gs.clone();
            next_game_state.apply_action(Action::SelectPiece(piece_name));
            next_game_state.board_state.preview_piece(
                gs.player_state.current_player,
                piece,
                position,
            );
            match next_game_state.play_previewed_piece() {
                Ok(_) => Some((
                    next_game_state.score.p2 - next_game_state.score.p1,
                    next_game_state,
                )),
                Err(_) => None,
            }
        })
        .max_by(|a, b| a.0.cmp(&b.0));

    match best_move_gs {
        Some((_score, next_gs)) => next_gs,
        None => {
            let mut next_gs = gs.clone();
            next_gs.apply_action(Action::PassTurn);
            next_gs
        }
    }
}

#[cfg(test)]
mod tests {

    use nalgebra::Vector3;

    use crate::{
        game_state::GameState,
        ts_interop::{Action, V3},
    };

    #[test]
    fn greedy_ai_test() {
        let mut game_state = GameState::new(crate::game_mode::GameMode::VSGreedyAI(
            crate::game_mode::TwoPlayerMap::Wall,
        ));
        println!("{:?}", game_state);

        game_state.apply_action(crate::ts_interop::Action::SelectPiece(
            crate::piece::PieceName::OneByThree,
        ));
        game_state.apply_action(Action::PreviewPiece(V3(Vector3::new(0.0, 0.0, 0.0))));
        game_state.apply_action(Action::PlayPreviewedPiece);

        println!("{:?}", game_state);
    }
}
