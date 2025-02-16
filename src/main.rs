mod components;
mod constants;
mod entities;
mod map;
use ggez::{conf, event, Context, GameResult};
use hecs::World;
use map::initialize_level;
use std::path;
mod systems;
use systems::input::run_input;
use systems::rendering::run_rendering;

struct Game {
    world: World,
}

// This is the main event loop. ggez tells us to implement
// two things: 1.updating 2.rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, context: &mut Context) -> Result<(), ggez::GameError> {
        // TODO: update game logic here
        {
            run_input(&self.world, context);
        }
        Ok(())
    }

    fn draw(&mut self, context: &mut Context) -> Result<(), ggez::GameError> {
        // TODO: update draw here
        {
            run_rendering(&self.world, context);
        }
        Ok(())
    }
}

pub fn main() -> GameResult {
    //NOTE: following code incorrect?
    // let world = World::new();
    let mut world = World::new();
    initialize_level(&mut world);

    // Create a game context and event loop
    let context_builder = ggez::ContextBuilder::new("rust_sokoban", "sokoban")
        .window_setup(conf::WindowSetup::default().title("Rust Sokoban!"))
        .window_mode(conf::WindowMode::default().dimensions(1500.0, 1500.0))
        .add_resource_path(path::PathBuf::from("./resources"));
    let (context, event_loop) = context_builder.build()?;

    // Create the game state
    let game = Game { world };

    // Run the main event loop
    event::run(context, event_loop, game)
}
