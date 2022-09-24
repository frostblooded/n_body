use ggez::*;
use rapier2d::prelude::{
    ColliderBuilder, ColliderHandle, RigidBody, RigidBodyBuilder, RigidBodyHandle,
};

use crate::{physics::Physics, vec2d::Vec2D};

pub const CIRCLE_RADIUS: f32 = 10.0;

pub struct Object {
    pub sprite: graphics::Mesh,
    pub position: Vec2D,
    pub rigid_body_handle: RigidBodyHandle,
    pub collider_handle: ColliderHandle,
}

impl Object {
    pub fn new(ctx: &mut Context, physics: &mut Physics, position: Vec2D) -> Self {
        let sprite: graphics::Mesh = graphics::Mesh::new_circle(
            ctx,
            graphics::DrawMode::fill(),
            Vec2D::ZERO,
            CIRCLE_RADIUS,
            0.1,
            graphics::Color::WHITE,
        )
        .expect("Failed to create a circle sprite");

        let rigid_body = RigidBodyBuilder::dynamic().translation(position.into());
        let rigid_body_handle = physics.rigid_body_set.insert(rigid_body);

        let collider = ColliderBuilder::ball(CIRCLE_RADIUS).build();
        let collider_handle = physics.collider_set.insert_with_parent(
            collider,
            rigid_body_handle,
            &mut physics.rigid_body_set,
        );

        Self {
            sprite,
            position,
            rigid_body_handle,
            collider_handle,
        }
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> Result<(), GameError> {
        canvas.draw(
            &self.sprite,
            graphics::DrawParam::default().dest_rect(self.position.into()),
        );

        Ok(())
    }

    pub fn get_rigid_body<'a>(&self, physics: &'a mut Physics) -> &'a mut RigidBody {
        &mut physics.rigid_body_set[self.rigid_body_handle]
    }

    pub fn update_position(&mut self, physics: &mut Physics) {
        self.position = self.get_rigid_body(physics).translation().into();
    }
}
