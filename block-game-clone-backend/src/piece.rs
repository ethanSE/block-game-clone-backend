//! Contains [Piece], [PieceName]
use core::f32::consts::PI;

use itertools::Itertools;
use nalgebra::{Rotation3, Vector3};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

use crate::ts_interop::RotationAxis;
extern crate alloc;
use alloc::{borrow::ToOwned, format, string::String, vec, vec::Vec};

/// Represents a piece as a Vec of offsets as [`nalgebra::Vector3<f32>`] from [0,0,0]
#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "pkg/types/Piece.ts")]
pub struct Piece {
    #[ts(type = "Array<[number,number,number]>")]
    pub coords: Vec<Vector3<f32>>,
}

impl Piece {
    /// applies vector translation to constituent cubes
    pub fn get_moved_copy(&self, position: Vector3<f32>) -> Piece {
        Piece {
            coords: self.coords.iter().map(|c| c + position).collect(),
        }
    }

    /// Imagine picking up a [Polycube](https://https://en.wikipedia.org/wiki/Polycube) by one of the cubes
    ///
    /// The position of the cubes are expressed as offsets from this selected cube which is at the origin
    pub fn set_origin(&mut self, new_origin: Vector3<f32>) {
        self.coords = self.coords.iter().map(|a| a - new_origin).collect();
    }

    /// Rotates the piece about the input RotationAxis
    pub fn rotate(&mut self, rotation_axis: RotationAxis) {
        let rotation = Rotation3::from_axis_angle(
            &match rotation_axis {
                RotationAxis::X => Vector3::x_axis(),
                RotationAxis::Y => Vector3::y_axis(),
            },
            PI / 2.0,
        );

        *self = self.apply_rotation(rotation)
    }

    fn apply_rotation(&self, rotation: Rotation3<f32>) -> Self {
        Self {
            coords: self
                .coords
                .iter()
                .map(|coord| {
                    let mut new_position = rotation * coord;
                    new_position.apply(|component| {
                        *component = component.round();
                    });
                    new_position
                })
                .collect(),
        }
    }

    pub fn from_vec_i8_array(i: Vec<[i8; 3]>) -> Piece {
        Piece {
            coords: i
                .iter()
                .map(|[x, y, z]| Vector3::new(*x as f32, *y as f32, *z as f32))
                .collect(),
        }
    }

    /// Does this piece support a given position. All cubes must be supported. A cube can be supported by another cube in the same piece
    pub fn supports(&self, position: &Vector3<f32>) -> bool {
        self.coords
            .iter()
            .any(|c| *c == position - Vector3::<f32>::new(0.0, 1.0, 0.0))
    }

    /// Give all possible rotations of the current piece
    pub fn get_available_piece_rotations(&self) -> Vec<Piece> {
        // determines which side faces upwards (positive Y direction)
        let side_up_rotations = vec![
            Rotation3::from_axis_angle(&Vector3::x_axis(), PI * 0.0),
            Rotation3::from_axis_angle(&Vector3::x_axis(), PI * 0.5),
            Rotation3::from_axis_angle(&Vector3::x_axis(), PI * 1.0),
            Rotation3::from_axis_angle(&Vector3::x_axis(), PI * 1.5),
            Rotation3::from_axis_angle(&Vector3::z_axis(), PI * 0.5),
            Rotation3::from_axis_angle(&Vector3::z_axis(), PI * 1.5),
        ];
        // rotation about y-axis
        let y_axis_rotations = vec![
            Rotation3::from_axis_angle(&Vector3::y_axis(), PI * 0.0),
            Rotation3::from_axis_angle(&Vector3::y_axis(), PI * 0.5),
            Rotation3::from_axis_angle(&Vector3::y_axis(), PI * 1.0),
            Rotation3::from_axis_angle(&Vector3::y_axis(), PI * 1.5),
        ];

        side_up_rotations
            .into_iter()
            .cartesian_product(y_axis_rotations.into_iter())
            .map(|(rot1, rot2)| self.apply_rotation(rot1).apply_rotation(rot2))
            .collect()
    }
}

/// identifies pieces
#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, TS, Debug, PartialOrd, Ord)]
#[ts(export, export_to = "pkg/types/PieceName.ts")]
#[serde(rename_all = "snake_case")]
pub enum PieceName {
    OneByTwo,
    OneByThree,
    OneByFour,
    TwoByTwo,
    Z,
    T,
    L,
    ShortL,
    RightScrew,
    LeftScrew,
    Corner,
}

#[cfg(test)]
mod tests {
    use std::f32::consts::PI;

    use nalgebra::{Rotation3, Vector3};

    use crate::piece::Piece;

    #[test]
    fn set_origin() {
        let mut a = Piece {
            coords: vec![Vector3::new(1.0, 1.0, 1.0), Vector3::new(2.0, 2.0, 2.0)],
        };

        let new_origin = Vector3::new(1.0, 1.0, 1.0);

        a.set_origin(new_origin);

        assert_eq!(a.coords[0], Vector3::new(0.0, 0.0, 0.0));
        assert_eq!(a.coords[1], Vector3::new(1.0, 1.0, 1.0))
    }

    #[test]
    fn v3_equality() {
        let a: Vector3<f32> = Vector3::<f32>::new(0.0, 0.0, 0.0);
        let b = Vector3::<f32>::new(0.0, 1.0, 0.0);
        assert_eq!(a, b - b);
    }

    #[test]
    fn vec_rounding() {
        let z_piece = Piece::from_vec_i8_array(vec![[0, 0, 0], [0, 0, 1], [0, 1, 1], [0, 1, 2]]);

        dbg!(&z_piece);

        let rotated_z_piece =
            z_piece.apply_rotation(Rotation3::from_axis_angle(&Vector3::x_axis(), PI));

        dbg!(rotated_z_piece);
    }
}
