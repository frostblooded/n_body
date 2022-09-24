use ggez::*;

use crate::object::{Object, CIRCLE_RADIUS};
use crate::physics::Physics;
use crate::vec2d::Vec2D;

pub const SCREEN_WIDTH: u32 = 1024;
pub const SCREEN_HEIGHT: u32 = 720;

pub struct World {
    pub objects: Vec<Object>,
    pub physics: Physics,
}

impl World {
    pub fn new() -> Self {
        World {
            objects: vec![],
            physics: Physics::new(),
        }
    }

    pub fn update(&mut self, _ctx: &mut Context, dt: std::time::Duration) -> Result<(), GameError> {
        const FORCE_MULTIPLIER: f32 = 6.6743e6f32;

        for (i, obj) in self.objects.iter().enumerate() {
            let rigid_body = obj.get_rigid_body(&mut self.physics);
            rigid_body.reset_forces(true);

            for (j, o) in self.objects.iter().enumerate() {
                if i == j {
                    continue;
                }

                let force = FORCE_MULTIPLIER / obj.position.sqr_dist_to(o.position);
                let velocity: Vec2D = (o.position - obj.position) * force * dt.as_secs_f32();
                rigid_body.add_force(velocity.into(), true);
            }
        }

        self.physics.step();

        for obj in self.objects.iter_mut() {
            obj.update_position(&mut self.physics);
        }

        Ok(())
    }

    pub fn draw(&self, canvas: &mut graphics::Canvas) -> Result<(), GameError> {
        for obj in &self.objects {
            obj.draw(canvas)?;
        }

        Ok(())
    }

    pub fn generate_world(&mut self, ctx: &mut Context) {
        const NUM_GENERATED_OBJECTS: u32 = 30;
        let mut added_objects_count: u32 = 0;

        while added_objects_count < NUM_GENERATED_OBJECTS {
            let random_position = Vec2D::random();

            if self.is_position_free(random_position) {
                let new_object = Object::new(ctx, &mut self.physics, random_position);
                self.objects.push(new_object);
                added_objects_count += 1;
            }
        }
    }

    pub fn is_position_free(&self, position: Vec2D) -> bool {
        let diameter: f32 = CIRCLE_RADIUS * 2.0;

        for obj in &self.objects {
            if obj.position.dist_to(position) < diameter {
                return false;
            }
        }

        true
    }
}
