use std::f32::consts::PI;

use crate::RotationAxis;
use nalgebra::{Rotation3, Vector3};
use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Serialize, Deserialize, Debug, Clone, TS)]
#[ts(export, export_to = "pkg/types/Piece.ts")]
pub struct Piece {
    #[ts(type = "Array<[number,number,number]>")]
    pub coords: Vec<Vector3<f32>>,
}

impl Piece {
    pub fn get_moved_copy(&self, position: Vector3<f32>) -> Piece {
        Piece {
            coords: self.coords.iter().map(|c| c + position).collect(),
        }
    }

    pub fn set_origin(&mut self, new_origin: Vector3<f32>) {
        self.coords = self
            .coords
            .iter()
            .map(|a| round_vec3(a - new_origin))
            .collect();
    }

    pub fn rotate(&mut self, rotation_axis: RotationAxis) {
        let axis = match rotation_axis {
            RotationAxis::X => Vector3::x_axis(),
            RotationAxis::Y => Vector3::y_axis(),
        };
        let angle = PI / 2.0;
        let rotation = Rotation3::from_axis_angle(&axis, angle);

        self.coords = self
            .coords
            .iter()
            .map(|a| round_vec3(rotation * a))
            .collect();
    }

    pub fn from_vec_i8_array(i: Vec<[i8; 3]>) -> Piece {
        Piece {
            coords: i
                .iter()
                .map(|[x, y, z]| Vector3::new(f32::from(*x), f32::from(*y), f32::from(*z)))
                .collect(),
        }
    }

    pub fn supports(&self, position: &Vector3<f32>) -> bool {
        self.coords
            .iter()
            .any(|c| *c == position - Vector3::<f32>::new(0.0, 1.0, 0.0))
    }
}

pub fn round_vec3(v3: Vector3<f32>) -> Vector3<f32> {
    Vector3::<f32>::new(f32::round(v3.x), f32::round(v3.y), f32::round(v3.z))
}

#[derive(Serialize, Deserialize, Hash, Eq, PartialEq, Copy, Clone, TS)]
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
    use nalgebra::Vector3;

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
}
