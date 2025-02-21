use crate::components::{BoxColor, Position};
use crate::entities::{create_box, create_box_spot, create_floor, create_player, create_wall};
use hecs::World;

pub fn initialize_level(world: &mut World) {
    const MAP: &str = "
    N N W W W W W W
    W W W . . . . W
    W . . . BB . . W
    W . . RB . . . W
    W . P . . . . W
    W . . . . RS . W
    W . . BS . . . W
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
                    create_wall(world, position);
                }
                "P" => {
                    create_floor(world, position);
                    create_player(world, position);
                }
                "BB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColor::Blue);
                }
                "RB" => {
                    create_floor(world, position);
                    create_box(world, position, BoxColor::Red);
                }
                "BS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColor::Blue);
                }
                "RS" => {
                    create_floor(world, position);
                    create_box_spot(world, position, BoxColor::Red);
                }
                "N" => (),
                c => panic!("unrecognized map item {}", c),
            }
        }
    }
}
