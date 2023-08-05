use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::piece::{Piece, PieceName};

#[derive(Serialize, Deserialize, TS)]
#[ts(export, export_to = "pkg/types/Hand.ts")]
pub struct Hand {
    one_by_two: Option<Piece>,
    one_by_three: Option<Piece>,
    one_by_four: Option<Piece>,
    two_by_two: Option<Piece>,
    z: Option<Piece>,
    t: Option<Piece>,
    l: Option<Piece>,
    short_l: Option<Piece>,
    right_screw: Option<Piece>,
    left_screw: Option<Piece>,
    corner: Option<Piece>,
}

impl Default for Hand {
    fn default() -> Self {
        Self {
            one_by_two: Some(Piece::from_vec_i8_array(vec![[0, 0, 0], [0, 0, 1]])),
            one_by_three: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 2],
            ])),
            one_by_four: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 2],
                [0, 0, 3],
            ])),
            two_by_two: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 0],
                [0, 1, 1],
            ])),
            z: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 1],
                [0, 1, 2],
            ])),
            t: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 1],
                [0, 0, 2],
            ])),
            l: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 0, 2],
                [0, 1, 2],
            ])),
            short_l: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 1],
            ])),
            right_screw: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 1],
                [1, 1, 1],
            ])),
            left_screw: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 1],
                [-1, 1, 1],
            ])),
            corner: Some(Piece::from_vec_i8_array(vec![
                [0, 0, 0],
                [0, 0, 1],
                [0, 1, 1],
                [1, 0, 1],
            ])),
        }
    }
}

impl Hand {
    pub fn get_mut(&mut self, index: PieceName) -> &mut Option<Piece> {
        match index {
            PieceName::OneByTwo => &mut self.one_by_two,
            PieceName::OneByThree => &mut self.one_by_three,
            PieceName::OneByFour => &mut self.one_by_four,
            PieceName::TwoByTwo => &mut self.two_by_two,
            PieceName::Z => &mut self.z,
            PieceName::T => &mut self.t,
            PieceName::L => &mut self.l,
            PieceName::ShortL => &mut self.short_l,
            PieceName::RightScrew => &mut self.right_screw,
            PieceName::LeftScrew => &mut self.left_screw,
            PieceName::Corner => &mut self.corner,
        }
    }
}
