use crate::prelude::*;

// The system requests read access to `Point` and `Render`
#[system]
#[read_component(Point)]
#[read_component(Render)]
pub fn entity_render(ecs: &SubWorld, #[resource] camera: &Camera) {
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);

    let offset = Point::new(camera.left_x, camera.top_y);

    // Query for all entities that have a `Point` and `Render` component.
    <(&Point, &Render)>::query()
        .iter(ecs)
        .for_each(|(pos, render)| {
            draw_batch.set(*pos - offset, render.color, render.glyph);
        });

    // 5000 is used as a sort order because the map may include up to 4000 elements. Thus leaving some room.
    draw_batch.submit(5000).expect("Batch error");
}
