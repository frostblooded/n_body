use std::ops::{Add, AddAssign, Mul, Sub};

use ggez::{graphics::Rect, *};
use rand::Rng;
use rapier2d::{
    na::{ArrayStorage, Const, Matrix},
    prelude::{nalgebra, vector, Vector},
};

use crate::world::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Clone, Copy, Debug)]
pub struct Vec2D {
    pub x: f32,
    pub y: f32,
}

impl Vec2D {
    pub const ZERO: Vec2D = Vec2D { x: 0.0, y: 0.0 };

    pub fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f32 = rng.gen::<f32>() * SCREEN_WIDTH as f32;
        let y: f32 = rng.gen::<f32>() * SCREEN_HEIGHT as f32;

        Self::new(x, y)
    }

    pub fn dist_to(&self, other: Self) -> f32 {
        self.sqr_dist_to(other).sqrt()
    }

    pub fn sqr_dist_to(&self, other: Self) -> f32 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

impl AddAssign<Vec2D> for Vec2D {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vec2D> for Vec2D {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Vec2D> for Vec2D {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Mul<f32> for Vec2D {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl From<Vec2D> for mint::Point2<f32> {
    fn from(v: Vec2D) -> Self {
        mint::Point2 {
            x: v.x as f32,
            y: v.y as f32,
        }
    }
}

impl From<Vec2D> for Rect {
    fn from(v: Vec2D) -> Self {
        Rect::new_i32(v.x as i32, v.y as i32, 1, 1)
    }
}

impl From<Vec2D> for Vector<f32> {
    fn from(v: Vec2D) -> Self {
        vector![v.x, v.y]
    }
}

impl From<&Matrix<f32, Const<2>, Const<1>, ArrayStorage<f32, 2, 1>>> for Vec2D {
    fn from(matrix: &Matrix<f32, Const<2>, Const<1>, ArrayStorage<f32, 2, 1>>) -> Self {
        Vec2D::new(matrix.x, matrix.y)
    }
}
