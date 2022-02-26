// Legion components are usually structs, but can also be enum types such as options.
// They don’t have to derive any functionality, it’s a good idea to derive `Clone`.
// This allows you to make a copy of the component if you need it.

pub use crate::prelude::*;

// Render component describes how the player appears on the screen
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Render {
    // bracket-lib class that stores both a foreground and background color in a single struct
    pub color: ColorPair,
    // bracket-lib class that stores a single character or glyph
    pub glyph: FontCharType,
}

// Player component is a “tag” indicating that an entity with this component is the player
// Component does not have to contain any fields.
// An empty component is called a "tag", serving as a flag that a property exists
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Player;
