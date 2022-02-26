use crate::prelude::*;

// System that renders the map
// This system does not use any components but must request read access to resources map and camera
#[system]
pub fn map_render(#[resource] map: &Map, #[resource] camera: &Camera) {
    // Start a drawing batch
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(0);

    for y in camera.top_y..=camera.bottom_y {
        for x in camera.left_x..camera.right_x {
            let pt = Point::new(x, y);
            let offset = Point::new(camera.left_x, camera.top_y);

            if map.in_bounds(pt) {
                let idx = map_index(x, y);
                let glyph = match map.tiles[idx] {
                    TileType::Floor => to_cp437('.'),
                    TileType::Wall => to_cp437('#'),
                };

                // Calling the draw batch instead of the context
                draw_batch.set(pt - offset, ColorPair::new(WHITE, BLACK), glyph);
            }
        }
    }

    // Submitting the batch add it to the global command list
    // 0 ensures the map is drawn at the beginning of the rendering cycle
    draw_batch.submit(0).expect("Batch error");
}

// Notes on batched rendering:
// bracket-lib offers a batching service so that we can request a new draw batch by calling `DrawBatch::new()`.
// This creates a buffer of deferred rendering commands.
// Draw commands won't be executed immediately but only after we've finished adding to the batch.
// Then, by calling draw_batch.submit(sort_order), we finalize the draw batch.
// `sort_order` specifies the order in which the command batches are executed, with 0 going first.
// Finally, in `tick()`, we render with `render_draw_buffer(ctx)`.
