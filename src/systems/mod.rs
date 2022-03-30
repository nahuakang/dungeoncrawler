mod collisions;
mod entity_render;
mod map_render;
mod player_input;
mod random_move;

use crate::prelude::*;

// This function creates a Legion Schedule — an execution plan for your systems.
// It follows the builder pattern: Schedule::builder starts the system-building process and build() finishes it.
pub fn build_schedule() -> Schedule {
    Schedule::builder()
        .add_system(player_input::player_input_system())
        .add_system(collisions::collisions_system())
        // Flush after collision detection ensures any deleted entities are gone bfore rendered
        // and all systems up to that point have finihsed executing before the next one runs
        .flush()
        .add_system(map_render::map_render_system())
        .add_system(entity_render::entity_render_system())
        .add_system(random_move::random_move_system())
        .build()
}

// Legion’s scheduler needs to know what types of component you are accessing,
// and how you need to access them. Multiple systems can access a read-only component at once,
// but only a single system can write to a component type at once
// (and prevents read-only access from running until it is finished—pre-venting
// a system from encountering data that changed partway through system execution).
