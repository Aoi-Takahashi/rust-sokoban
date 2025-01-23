use ::ggez::{
    conf, event,
    graphics::{self, DrawParam, Image},
    Context, GameResult,
};
use ::std::path;
use ggez::glam::Vec2;
use hecs::{Entity, World};

const TILE_WIDTH: f32 = 32.0;

#[allow(dead_code)]
struct Game {
    world: World,
}

#[allow(dead_code)]
#[derive(Clone)]
pub struct Position {
    x: u8,
    y: u8,
    z: u8,
}

#[allow(dead_code)]
pub struct Renderable {
    path: String,
}

pub struct Wall {}

pub struct Player {}

pub struct Box {}

pub struct BoxSpot {}

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/wall.png".to_string(),
        },
        Wall {},
    ))
}

pub fn create_floor(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 5, ..position },
        Renderable {
            path: "/images/floor.png".to_string(),
        },
    ))
}

pub fn create_box(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/box.png".to_string(),
        },
        Box {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: "/images/box_spot.png".to_string(),
        },
        BoxSpot {},
    ))
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/player.png".to_string(),
        },
        Player {},
    ))
}

// Initialize the level
pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . B . . W
    W . . . . . . W
    W . P . . . . W
    W . . . . . . W
    W . . S . . . W
    W . . . . . . W
    W W W W W W W W
    ";

    load_map(world, MAP.to_string());
}

pub fn load_map(world: &mut World, map_string: String) {
    // read all lines
    let rows: Vec<&str> = map_string.trim().split('\n').map(|x| x.trim()).collect();

    for (y, row) in rows.iter().enumerate() {
        println!("(y:{y} row:{row})");
        let columns: Vec<&str> = row.split(' ').collect();

        for (x, column) in columns.iter().enumerate() {
            println!("(x:{x} column:{column})");
            // Create the position at which to create something on the map
            let position = Position {
                x: x as u8,
                y: y as u8,
                z: 0, // we will get the z from the factory functions
            };
            //NOTE: If you don't have the following code it won't work as expected
            let position_clone = position.clone();

            // Figure out what object we should create
            match *column {
                "." => {
                    create_floor(world, position);
                }
                "W" => {
                    create_floor(world, position);
                    create_wall(world, position_clone);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position_clone);
                }
                "B" => {
                    create_floor(world, position);
                    create_box(world, position_clone);
                }
                "S" => {
                    create_floor(world, position);
                    create_box_spot(world, position_clone);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}

//Here is the implementation of the rendering system. It does a few things:
// ・clear the screen (ensuring we don't keep any of the state rendered on the previous frame)
// ・get all entities with a renderable component and sort them by z (we do this in order to ensure we can render things on top of each other, for example the player should be above the floor, otherwise we wouldn't be able to see them)
// ・iterate through sorted entities and render each of them as an image
// ・finally, present to the screen

pub fn run_rendering(world: &World, context: &mut Context) {
    // 1.Clearing the screen (this gives us the background colour)
    let mut canvas =
        graphics::Canvas::from_frame(context, graphics::Color::from([0.95, 0.95, 0.95, 1.0]));

    // 2.Get all the renderables with their positions and sort by the position z
    // This will allow us to have entities layered visually.
    let mut query = world.query::<(&Position, &Renderable)>();
    let mut rendering_data: Vec<(Entity, (&Position, &Renderable))> = query.into_iter().collect();
    rendering_data.sort_by_key(|&k| k.1 .0.z); //NOTE: 「k.1.0.z」 は ネストしたTupleのIndexアクセス

    // 3.Iterate through all pairs of positions & renderables, load the image
    // and draw it at the specified position.
    for (_, (position, renderable)) in rendering_data.iter() {
        // Load the image
        let image = Image::from_path(context, renderable.path.clone()).unwrap();
        let x = position.x as f32 * TILE_WIDTH;
        let y = position.y as f32 * TILE_WIDTH;

        // draw
        let draw_params = DrawParam::new().dest(Vec2::new(x, y));
        canvas.draw(&image, draw_params);
    }

    // 4.Finally, present the canvas, this will actually display everything
    // on the screen.
    canvas.finish(context).expect("expected to present");
}

// This is the main event loop. ggez tells us to implement
// two things: 1.updating 2.rendering
impl event::EventHandler<ggez::GameError> for Game {
    fn update(&mut self, _context: &mut Context) -> Result<(), ggez::GameError> {
        // TODO: update game logic here
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
