use crate::{
    components::{Box, BoxColor, BoxSpot, Immovable, Movable, Player, Position, Renderable, Wall},
    resources::Gameplay,
};
use hecs::{Entity, World};

pub fn create_wall(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/wall.png".to_string(),
        },
        Wall {},
        Immovable {},
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

pub fn create_box(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: format!("/images/box_{}.png", color),
        },
        Box { color },
        Movable {},
    ))
}

pub fn create_box_spot(world: &mut World, position: Position, color: BoxColor) -> Entity {
    world.spawn((
        Position { z: 9, ..position },
        Renderable {
            path: format!("/images/box_{}.png", color),
        },
        BoxSpot { color },
    ))
}

pub fn create_player(world: &mut World, position: Position) -> Entity {
    world.spawn((
        Position { z: 10, ..position },
        Renderable {
            path: "/images/player.png".to_string(),
        },
        Player {},
        Movable {},
    ))
}

pub fn create_gameplay(world: &mut World) -> Entity {
    world.spawn((Gameplay::default(),))
}
