use crate::prelude::*;

// Proc macro #[system] to transforms `player_input` function to `player_input_system`
// Proc macro #[write_component] requests writable access to a component type, in this case, the `Point` component.
// You must request write access if you intend to change the contents of a component in your system.
// Proc macro #[read_component] requests read-only access to a component type.
// You must request read access to use the values stored in a component of this type.
// `SubWorld` is like a `World` but only sees the components you requested
// Proc macro #[resource] requests access to types you stored in Legion’s Resource handler.
// We request read-access on map but mutable access to camera
#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] camera: &mut Camera,
) {
    if let Some(key) = key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            _ => Point::new(0, 0),
        };

        if delta.x != 0 || delta.y != 0 {
            // Queries list one or more components, and return references—mutable if you use &mut to each instance of that component type.
            // Legion queries include a filter() function to further refine the set of components required for a query to match an entity.
            // Filter specifies that only entities with a Point component and a Player tag component should be included in the query.
            let mut players = <&mut Point>::query().filter(component::<Player>());

            players.iter_mut(ecs).for_each(|pos| {
                let destination = *pos + delta;
                if map.can_enter_tile(destination) {
                    *pos = destination;
                    camera.on_player_move(destination);
                }
            })
        }
    }
}
