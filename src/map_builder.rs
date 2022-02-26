use crate::prelude::*;

const NUM_ROOMS: usize = 20;

pub struct MapBuilder {
    pub map: Map,
    pub rooms: Vec<Rect>, // Rooms will be added to the map
    pub player_start: Point,
}

impl MapBuilder {
    pub fn new(rng: &mut RandomNumberGenerator) -> Self {
        let mut map_builder = MapBuilder {
            map: Map::new(),
            rooms: Vec::new(),
            player_start: Point::zero(),
        };

        map_builder.fill(TileType::Wall);
        map_builder.build_random_rooms(rng);
        map_builder.build_corridors(rng);
        map_builder.player_start = map_builder.rooms[0].center();

        map_builder
    }

    // Helper method for building "dog-leg" corridors of vertical section, joined
    // by a single corner. It iterates from the smaller value of the
    // 2 y coordinates given and carve the tunnel.
    fn apply_vertical_tunnel(&mut self, y1: i32, y2: i32, x: i32) {
        use std::cmp::{max, min};
        for y in min(y1, y2)..=max(y1, y2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    // Helper method for building "dog-leg" corridors of horizontal section, joined
    // by a single corner. It iterates from the smaller value of the
    // 2 x coordinates given and carve the tunnel.
    fn apply_horizontal_tunnel(&mut self, x1: i32, x2: i32, y: i32) {
        use std::cmp::{max, min};
        for x in min(x1, x2)..=max(x1, x2) {
            if let Some(index) = self.map.try_index(Point::new(x, y)) {
                self.map.tiles[index as usize] = TileType::Floor;
            }
        }
    }

    // Build "dog-leg" corridors between rooms.
    fn build_corridors(&mut self, rng: &mut RandomNumberGenerator) {
        let mut rooms = self.rooms.clone();

        // Sort the rooms by their center point before allocating corridors
        // makes it more likely that corridors will connect adjacent rooms
        // and not snake across the whole map
        rooms.sort_by(|a, b| a.center().x.cmp(&b.center().x));

        for (i, room) in rooms.iter().enumerate().skip(1) {
            // Obtain the center point of both rooms
            // (This would be invalid if we did not skip 1)
            let prev = rooms[i - 1].center();
            let new = room.center();

            // Either of these two ensures that two rooms are connected
            // Difference is if the tunnel starts with horizontal or vertical corridors
            // before the other is built to connect the two rooms
            if rng.range(0, 2) == 1 {
                self.apply_horizontal_tunnel(prev.x, new.x, prev.y);
                self.apply_vertical_tunnel(prev.y, new.y, new.x);
            } else {
                self.apply_vertical_tunnel(prev.y, new.y, prev.x);
                self.apply_horizontal_tunnel(prev.x, new.x, new.y);
            }
        }
    }

    // From a solid map of `TileType::Wall`, carve out random, non-overlapping rooms.
    fn build_random_rooms(&mut self, rng: &mut RandomNumberGenerator) {
        // Keep generating random rooms until there are `NUM_ROOMS` rooms
        while self.rooms.len() < NUM_ROOMS {
            // Create a randomly positioned room of random sizes
            let room = Rect::with_size(
                rng.range(1, SCREEN_WIDTH - 10),
                rng.range(1, SCREEN_HEIGHT - 10),
                rng.range(2, 10),
                rng.range(2, 10),
            );

            // Check that the newly created random room does not overlap existing rooms
            let mut overlap = false;
            for r in self.rooms.iter() {
                if r.intersect(&room) {
                    overlap = true;
                }
            }

            // If the newly created room fits the map, carve it out
            if !overlap {
                room.for_each(|p| {
                    if p.x > 0 && p.x < SCREEN_WIDTH && p.y > 0 && p.y < SCREEN_HEIGHT {
                        let index = map_index(p.x, p.y);
                        self.map.tiles[index] = TileType::Floor;
                    }
                });
                self.rooms.push(room);
            }
        }
    }

    // Room-carving helper method to fill the map with a specific tile types.
    // This method supports room-carving and is used to fill the map
    // with only `TileType::Wall` first.
    pub fn fill(&mut self, tile: TileType) {
        self.map.tiles.iter_mut().for_each(|t| *t = tile);
    }
}
