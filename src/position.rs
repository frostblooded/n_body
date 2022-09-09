use std::ops::{Add, AddAssign, Mul, Sub};

use ggez::{graphics::Rect, *};
use rand::Rng;

use crate::world::{SCREEN_HEIGHT, SCREEN_WIDTH};

#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub x: f64,
    pub y: f64,
}

impl Position {
    pub fn new(x: f64, y: f64) -> Self {
        Position { x, y }
    }

    pub fn random() -> Self {
        let mut rng = rand::thread_rng();
        let x: f64 = rng.gen::<f64>() * SCREEN_WIDTH as f64;
        let y: f64 = rng.gen::<f64>() * SCREEN_HEIGHT as f64;

        Position::new(x, y)
    }

    pub fn dist_to(&self, other: Position) -> f64 {
        self.sqr_dist_to(other).sqrt()
    }

    pub fn sqr_dist_to(&self, other: Position) -> f64 {
        (self.x - other.x).powi(2) + (self.y - other.y).powi(2)
    }
}

impl AddAssign<Position> for Position {
    fn add_assign(&mut self, rhs: Position) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Position> for Position {
    type Output = Position;

    fn add(mut self, rhs: Position) -> Self::Output {
        self += rhs;
        self
    }
}

impl Sub<Position> for Position {
    type Output = Position;

    fn sub(mut self, rhs: Position) -> Self::Output {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self
    }
}

impl Mul<f64> for Position {
    type Output = Position;

    fn mul(mut self, rhs: f64) -> Self::Output {
        self.x *= rhs;
        self.y *= rhs;
        self
    }
}

impl From<Position> for mint::Point2<f32> {
    fn from(position: Position) -> Self {
        mint::Point2 {
            x: position.x as f32,
            y: position.y as f32,
        }
    }
}

impl From<Position> for Rect {
    fn from(position: Position) -> Self {
        Rect::new_i32(position.x as i32, position.y as i32, 1, 1)
    }
}
