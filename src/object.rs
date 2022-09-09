use ggez::*;

use crate::position::Position;

pub const CIRCLE_RADIUS: f32 = 10.0;

pub struct Object {
    pub sprite: graphics::Mesh,
    pub position: Position,
}

impl Object {
    pub fn new(ctx: &mut Context, position: Position) -> Self {
        let sprite: graphics::Mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Position::new(0.0, 0.0),
            CIRCLE_RADIUS,
            0.1,
            graphics::Color::WHITE,
        )
        .expect("Failed to create a circle sprite");

        Self { sprite, position }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> Result<(), GameError> {
        canvas.draw(
            &self.sprite,
            graphics::DrawParam::default().dest_rect(self.position.into()),
        );

        Ok(())
    }
}
