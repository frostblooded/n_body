use ggez::*;

use crate::world::World;

pub struct State {
    dt: std::time::Duration,
    pub world: World,
}

impl State {
    pub fn new() -> Self {
        Self {
            dt: std::time::Duration::ZERO,
            world: World::new(),
        }
    }
}

impl ggez::event::EventHandler<GameError> for State {
    fn update(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        self.dt = ctx.time.delta();
        self.world.update(ctx, self.dt)?;
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), GameError> {
        println!("Frame time = {}ms", self.dt.as_millis());

        let mut canvas: graphics::Canvas =
            graphics::Canvas::from_frame(ctx, graphics::Color::BLACK);

        self.world.draw(&mut canvas)?;
        canvas.finish(ctx)?;

        Ok(())
    }
}
