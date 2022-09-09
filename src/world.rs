use ggez::*;

use crate::object::{Object, CIRCLE_RADIUS};
use crate::position::Position;

pub const SCREEN_WIDTH: u32 = 1024;
pub const SCREEN_HEIGHT: u32 = 720;

pub struct World {
    pub objects: Vec<Object>,
}

impl World {
    pub fn new() -> Self {
        World { objects: vec![] }
    }

    pub fn update(&mut self, _ctx: &mut Context, dt: std::time::Duration) -> Result<(), GameError> {
        const GRAVITATIONAL_CONSTANT: f64 = 6.6743e1f64;
        let mut wanted_movements = vec![Position::new(0.0, 0.0); self.objects.len()];

        for (i, obj) in self.objects.iter().enumerate() {
            for (j, o) in self.objects.iter().enumerate() {
                if i == j {
                    continue;
                }

                let force = GRAVITATIONAL_CONSTANT / obj.position.sqr_dist_to(o.position);
                let velocity: Position = (o.position - obj.position) * force * dt.as_secs_f64();
                wanted_movements[i] += velocity;
            }
        }

        for (i, obj) in self.objects.iter_mut().enumerate() {
            obj.position += wanted_movements[i];
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
        const NUM_GENERATED_OBJECTS: u32 = 10;
        let mut added_objects_count: u32 = 0;

        while added_objects_count < NUM_GENERATED_OBJECTS {
            let random_position = Position::random();

            if self.is_position_free(random_position) {
                dbg!("Approved position: {:?}", random_position);
                self.objects.push(Object::new(ctx, random_position));
                added_objects_count += 1;
            }
        }
    }

    pub fn is_position_free(&self, position: Position) -> bool {
        let diameter: f32 = CIRCLE_RADIUS * 2.0;

        for obj in &self.objects {
            if obj.position.dist_to(position) < diameter.into() {
                return false;
            }
        }

        true
    }
}
