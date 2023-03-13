// use crate::wasm4::*;

// pub(crate) const TILE_PIXELS: i32 = 16;
// pub(crate) const PLAYER_SIZE: i32 = 1;  // in units of tiles
// pub(crate) const CRATE_SIZE: i32 = 1;  // in units of tiles
// pub(crate) const LEVEL_HEIGHT_SIZE: i32 = 10;

// const NUM_SPOTS: i32 = (
//     (
//         SCREEN_SIZE as i32 / (
//             TILE_PIXELS * CRATE_SIZE
//         ) - 2
//     ) * LEVEL_HEIGHT_SIZE
// ) / 8;  // for level height size == 10, == 10

// fn is_solid(tile_x: i32, tile_y: i32) -> bool {
//     if tile_x == 0 || tile_x == SCREEN_SIZE as i32 / TILE_PIXELS - 1 {
//         true
//     } else {
//         let index = tile_y as usize;
//         let val = MAP[index];
//         val & (1 << tile_x) != 0
//     }
// }

