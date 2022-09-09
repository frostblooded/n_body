mod object;
mod position;
mod state;
mod world;

use ggez::{conf::WindowMode, *};

use crate::state::State;

fn main() {
    let mut state = State::new();

    let conf: conf::Conf = conf::Conf::new();
    let window_mode: WindowMode =
        WindowMode::default().dimensions(world::SCREEN_WIDTH as f32, world::SCREEN_HEIGHT as f32);

    let (mut ctx, event_loop) = ContextBuilder::new("hello_ggez", "Nikolay Danailov")
        .default_conf(conf)
        .window_mode(window_mode)
        .build()
        .expect("Failed to build a configuration");

    state.world.generate_world(&mut ctx);

    event::run(ctx, event_loop, state);
}
