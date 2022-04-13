use super::map::{Map, Tile};

pub fn mouse_collision_tile(mouse_x: u32, mouse_y: u32, map: &Map) -> Option<Tile> {
    for tile in map.tiles.iter() {
        if mouse_x >= tile.x
            && mouse_x <= tile.x + 32
            && mouse_y >= tile.y
            && mouse_y <= tile.y + 32
        {
            return Some(tile.clone());
        }
    }
    None
}
