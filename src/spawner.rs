use crate::prelude::*;

// Spawn a player in the given World at the given position
pub fn spawn_player(ecs: &mut World, pos: Point) {
    // Calling push() creates a new Entity composed of the listed components.
    // The components are separated in a tuple.
    ecs.push((
        Player,
        pos,
        Render {
            color: ColorPair::new(WHITE, BLACK),
            glyph: to_cp437('@'),
        },
    ));
}
