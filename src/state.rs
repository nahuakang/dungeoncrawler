pub use crate::prelude::*;

pub struct State {
    ecs: World,
    resources: Resources,
    systems: Schedule,
}

impl State {
    pub fn new() -> Self {
        // legion stores all entities and components in the `World` struct
        let mut ecs = World::default();
        let mut resources = Resources::default();
        let mut rng = RandomNumberGenerator::new();
        let map_builder = MapBuilder::new(&mut rng);

        // Calling spawn_player to add the player and their components to the ECS
        spawn_player(&mut ecs, map_builder.player_start);

        // Spawn monsters in each room except for the first room the player is in
        map_builder
            .rooms
            .iter()
            .skip(1)
            .map(|r| r.center())
            .for_each(|pos| spawn_monster(&mut ecs, &mut rng, pos));

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

        // Execute systems (which also submits draw buffers)
        self.systems.execute(&mut self.ecs, &mut self.resources);

        // Render draw buffers
        render_draw_buffer(ctx).expect("Render error");
    }
}
