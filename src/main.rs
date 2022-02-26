mod camera;
mod components;
mod map;
mod map_builder;
mod spawner;
mod systems;

// Use prelude to export common functionality of the crate
// and external libraries to the rest of the program.
mod prelude {
    pub use bracket_lib::prelude::*;
    pub use legion::systems::CommandBuffer;
    pub use legion::world::SubWorld;
    pub use legion::*;
    pub const SCREEN_HEIGHT: i32 = 50;
    pub const SCREEN_WIDTH: i32 = 80;
    pub const DISPLAY_WIDTH: i32 = SCREEN_WIDTH / 2;
    pub const DISPLAY_HEIGHT: i32 = SCREEN_HEIGHT / 2;
    pub use crate::camera::*;
    pub use crate::components::*;
    pub use crate::map::*;
    pub use crate::map_builder::*;
    pub use crate::spawner::*;
    pub use crate::systems::*;
}

use prelude::*;

struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    fn new() -> Self {
        // legion stores all entities and components in the `World` struct
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Calling spawn_player to add the player and their components to the ECS
        spawn_player(&mut ecs, map_builder.player_start);

        // Map and camera are part of our resources list
        resources.insert(map_builder.map);
        resources.insert(Camera::new(map_builder.player_start));

        Self {
            ecs,
            resources,
            systems: build_schedule(),
        }
    }
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        // Set active console to map layer and clear
        ctx.set_active_console(0);
        ctx.cls();

        // Set active console to player layer and clear
        ctx.set_active_console(1);
        ctx.cls();

        // Add ctx.key (which holds the keyboard state) as a resource
        // to make the current keyboard state available to any system that requests it
        // When a resource is inserted into Legion’s resource handler, it replaces any existing resource of the same type
        self.resources.insert(ctx.key);

        // Execute systems
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // TODO: Render draw buffer
    }
}

fn main() -> BError {
    let ctx = BTermBuilder::new()
        .with_title("Dungeon Crawler")
        // Tracks game speed and informs OS to rest between frames
        .with_fps_cap(30.0)
        // Use display sizes instead of screen sizes
        .with_dimensions(DISPLAY_WIDTH, DISPLAY_HEIGHT)
        // The tile dimensions are the size of each char in font file
        .with_tile_dimensions(32, 32)
        .with_resource_path("resources/")
        .with_font("dungeonfont.png", 32, 32)
        .with_simple_console(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        // Add a second console with no background so transparency shows through it.
        .with_simple_console_no_bg(DISPLAY_WIDTH, DISPLAY_HEIGHT, "dungeonfont.png")
        .build()?;

    main_loop(ctx, State::new())
}

// Note on `to_cp437`
// Looking at the existing font files, they are mapped to the old "codepage 437" map.
// So ASCII characters are in the same position and there's a few special characters.
// They are indexed with 0 being the top-left, 255 the bottom-right:
// Going 0..15 on the first row, 16..31 on the second and so on.
// So you can replace the glyph in slot 2 and use to_cp437(2) to render that character.
