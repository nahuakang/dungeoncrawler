use crate::prelude::*;

pub struct Player {
    pub position: Point,
}

impl Player {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    // Render the player's position in camera view.
    pub fn render(&self, ctx: &mut BTerm, camera: &Camera) {
        // Set the context to console 1, a.k.a. the player layer
        ctx.set_active_console(1);
        ctx.set(
            self.position.x - camera.left_x,
            self.position.y - camera.top_y,
            WHITE,
            BLACK,
            to_cp437('@'),
        );
    }

    // Update the player position on the camera view given keyboard input.
    pub fn update(&mut self, ctx: &mut BTerm, map: &Map, camera: &mut Camera) {
        if let Some(key) = ctx.key {
            let delta = match key {
                VirtualKeyCode::Left => Point::new(-1, 0),
                VirtualKeyCode::Right => Point::new(1, 0),
                VirtualKeyCode::Up => Point::new(0, -1),
                VirtualKeyCode::Down => Point::new(0, 1),
                _ => Point::zero(),
            };

            let new_position = self.position + delta;

            if map.can_enter_tile(new_position) {
                self.position = new_position;
                // Update camera position after player position changed
                camera.on_player_move(new_position);
            }
        }
    }
}
