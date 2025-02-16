//Here is the implementation of the rendering system. It does a few things:
// ・clear the screen (ensuring we don't keep any of the state rendered on the previous frame)
// ・get all entities with a renderable component and sort them by z (we do this in order to ensure we can render things on top of each other, for example the player should be above the floor, otherwise we wouldn't be able to see them)
// ・iterate through sorted entities and render each of them as an image
// ・finally, present to the screen

use ggez::{
    glam::Vec2,
    graphics::{self, DrawParam, Image},
    Context,
};
use hecs::{Entity, World};

use crate::{
    components::{Position, Renderable},
    constants::TILE_WIDTH,
};

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
