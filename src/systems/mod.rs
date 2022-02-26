use crate::prelude::*;

// This function creates a Legion Schedule—an execution plan for your systems.
// It follows the builder pattern: Schedule::builder starts the system-building process
// and build() finishes it.
pub fn build_schedule() -> Schedule {
    Schedule::builder().build()
}
