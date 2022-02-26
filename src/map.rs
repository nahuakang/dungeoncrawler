use crate::prelude::*;
const NUM_TILES: usize = (SCREEN_WIDTH * SCREEN_HEIGHT) as usize;

#[derive(Clone, Copy, PartialEq)]
pub enum TileType {
    Floor,
    Wall,
}

pub struct Map {
    pub tiles: Vec<TileType>,
}

// Map uses row-first indexing, so a 5x3 map would be indexed as:
// |  0 |  1 |  2 |  3 |  4 |
// |  5 |  6 |  7 |  8 |  9 |
// | 10 | 11 | 12 | 13 | 14 |
// index = (y * WIDTH) + x;
// x = index % WIDTH;
// y = index / WIDTH:
impl Map {
    pub fn new() -> Self {
        Self {
            tiles: vec![TileType::Floor; NUM_TILES],
        }
    }

    // Check that the player can enter the given point's tile.
    pub fn can_enter_tile(&self, point: Point) -> bool {
        self.in_bounds(point) && self.tiles[map_index(point.x, point.y)] == TileType::Floor
    }

    // Check that the given point is within the map boundaries.
    pub fn in_bounds(&self, point: Point) -> bool {
        point.x >= 0 && point.x < SCREEN_WIDTH && point.y >= 0 && point.y < SCREEN_HEIGHT
    }

    // Return the given tile's index coordinates or return a `None` option if
    // the requested coordinates fall outside of the map boundaries.
    pub fn try_index(&self, point: Point) -> Option<usize> {
        if !self.in_bounds(point) {
            None
        } else {
            Some(map_index(point.x, point.y))
        }
    }
}

// Map a given coordinate into index number in a row-first indexing schema.
pub fn map_index(x: i32, y: i32) -> usize {
    ((y * SCREEN_WIDTH) + x) as usize
}
