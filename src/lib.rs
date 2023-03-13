// #![no_std]
#[cfg(feature = "buddy-alloc")]
mod alloc;
mod wasm4;
use wasm4::*;
// use core::panic;

// #[panic_handler]
// fn ph(_info: &panic::PanicInfo) -> ! {
//     loop {}
// }

// choose character

// 

// enum Character {
//     Kangaroo,
//     Frog,
//     Cricket,
//     Rabbit,
// }

pub(crate) const KANGAROO_COLORS: u16 = 0x4230;
pub(crate) const KANGAROO_WIDTH: u32 = 16;
pub(crate) const KANGAROO_HEIGHT: u32 = 16;
pub(crate) const KANGAROO_FLAGS: u32 = BLIT_2BPP;
pub(crate) const KANGAROO_SPRITE: [u8; 128] = [0, 0, 16, 0, 0, 64, 100, 0, 1, 144, 25, 80, 6, 144, 6, 228, 26, 64, 6, 173, 105, 1, 86, 169, 105, 6, 170, 148, 100, 26, 170, 144, 105, 26, 165, 144, 105, 106, 165, 144, 106, 170, 151, 144, 26, 170, 65, 64, 6, 170, 64, 0, 1, 90, 84, 0, 0, 26, 164, 0, 0, 21, 84, 0, 1, 64, 1, 80, 6, 144, 90, 228, 26, 65, 166, 173, 105, 0, 86, 169, 105, 5, 170, 148, 100, 26, 170, 144, 104, 26, 165, 144, 105, 106, 165, 144, 106, 170, 151, 144, 26, 170, 65, 64, 6, 170, 64, 0, 1, 90, 64, 0, 0, 25, 0, 0, 0, 25, 0, 0, 0, 25, 0, 0, 0, 4, 0, 0];
pub(crate) fn draw_kangaroo(x: i32, y: i32, animation_frame: usize, additional_flags: u32) {
    unsafe { *DRAW_COLORS = KANGAROO_COLORS };
    blit(
        &KANGAROO_SPRITE[animation_frame*64..(animation_frame+1)*64],
        x,
        y,
        KANGAROO_WIDTH,
        KANGAROO_HEIGHT,
        KANGAROO_FLAGS | additional_flags
    );
}

pub(crate) const CRATE_COLORS: u16 = 0x4320;
pub(crate) const CRATE_WIDTH: u32 = 16;
pub(crate) const CRATE_HEIGHT: u32 = 16;
pub(crate) const CRATE_FLAGS: u32 = BLIT_2BPP;
pub(crate) const CRATE_SPRITE: [u8; 64] = [250, 170, 170, 175, 249, 85, 85, 111, 174, 85, 85, 186, 155, 149, 86, 230, 150, 229, 91, 150, 149, 185, 110, 86, 149, 110, 185, 86, 149, 91, 229, 86, 149, 91, 229, 86, 149, 110, 185, 86, 149, 185, 110, 86, 150, 229, 91, 150, 155, 149, 86, 230, 174, 85, 85, 186, 249, 85, 85, 111, 250, 170, 170, 175];
pub(crate) fn draw_crate(x: i32, y: i32, additional_flags: u32) {
    unsafe { *DRAW_COLORS = CRATE_COLORS };
    blit(&CRATE_SPRITE, x, y, CRATE_WIDTH, CRATE_HEIGHT, CRATE_FLAGS | additional_flags);
}


// struct Game {
//     frame: u32,
//     gamepad: u8,
//     previous_gamepad: u8,
//     // pressed_this_frame: u8,
//     face_left: bool,
//     player_x: i32,
//     player_y: i32,
//     player_vy: i32,
//     lava_height: i32,
//     camera_top: i32,
//     grounded: bool,
//     walking: bool,
// }

fn jump_sound() {
    tone(
        170 | (360 << 16),  // freq1 | freq2
        2 | (2 << 8) | (2 << 16) | (2 << 24),  // sustain | release | decay | attack
        100 | (62 << 8),  // volume | peak
        TONE_TRIANGLE,
    );
}



pub(crate) const TILE_PIXELS: i32 = 16;
// pub(crate) const PLAYER_SIZE: i32 = 1;  // in units of tiles
pub(crate) const CRATE_SIZE: i32 = 1;  // in units of tiles
pub(crate) const LEVEL_HEIGHT_SIZE: i32 = 10;

const NUM_SPOTS: i32 = (
    (
        SCREEN_SIZE as i32 / (
            TILE_PIXELS * CRATE_SIZE
        ) - 2
    ) * LEVEL_HEIGHT_SIZE
) / 8;  // for level height size == 10, == 10

trait UpdateDraw {
    fn update(&mut self, level: &Level);
    fn draw(&self, level: &Level);
}

#[derive(Clone, Copy)]
pub(crate) struct XY<T> {
    pub x: T,
    pub y: T,
}

// enum VictoryStatus {
//     Playing,
//     Win,
//     Lose,
// }

struct Player {
    // character: Character,
    gamepad: u8,
    previous_gamepad: u8,
    face_left: bool,
    position: XY<i32>,
    velocity: XY<f32>,
    grounded: bool,
    walking: bool,
    // victory_status: VictoryStatus,
}

pub(crate) struct Map {
    pub blocks: [u8; NUM_SPOTS as usize],
}

impl Map {
    pub fn is_solid(&self, tile_x: i32, tile_y: i32) -> bool {
        if tile_x == 0 || tile_x == SCREEN_SIZE as i32 / TILE_PIXELS - 1 {
            true
        } else {
            let index = tile_y as usize;
            if let Some(&val) = self.blocks.get(index) {
                val & (1 << (8 - tile_x)) != 0
            } else {
                false
            }
        }
    }
}

// struct Goal {
//     position: XY,
// }

// struct Lava {
//     height: i32,
// }

// struct Camera {
//     height: i32,
// }

pub(crate) struct Level {
    pub player: Player,
    pub map: Map,
    // pub goal: Goal,
    // pub lava: Lava,
    // pub camera: Camera,
}

const ACCELERATION: XY<f32> = XY { x: 0.0, y: 0.3 };

impl Level {
    fn update(&mut self) {
        // update for new frame
        self.player.walking = false;
        self.player.velocity.x = 0.0;
        self.player.previous_gamepad = self.player.gamepad;
        self.player.gamepad = unsafe { *GAMEPAD1 };

        // check for button presses
        let vy = if self.player.grounded && self.player.gamepad & (self.player.gamepad ^ self.player.previous_gamepad) & BUTTON_UP != 0 {
            self.player.grounded = false;
            jump_sound();
            -4.0
        } else {
            self.player.velocity.y
        };

        let vx = if self.player.gamepad & BUTTON_LEFT != 0 {
            self.player.face_left = true;
            -2
        } else if self.player.gamepad & BUTTON_RIGHT != 0 {
            self.player.face_left = false;
            2
        } else {
            0
        };

        // check for collisions with blocks
        // if moving up
        self.adjust_position(vx, vy);
        
        // if !self.player.grounded {
        //     self.player.velocity.y += ACCELERATION.y;
        // }
        // check for collisions with goal
    }

    fn adjust_position(&mut self, vx: i32, vy: f32) {
        // use vx to check if they are clipping into a wall
        /*
          t     u
        a ┌─┐   ┌─┐
          | | x | |
          └─┘   └─┘
        */
        // moving left checks
        let t = self.player.position.x / TILE_PIXELS - 1;
        let a = self.player.position.y / TILE_PIXELS;
        let b = (self.player.position.y + TILE_PIXELS - 1) / TILE_PIXELS;
        let g = self.map.is_solid(t, a) || self.map.is_solid(t, b);

        // moving right checks
        let u = (self.player.position.x + TILE_PIXELS) / TILE_PIXELS;
        let h = self.map.is_solid(u, a) || self.map.is_solid(u, a);

        if g && self.player.position.x + vx < (t + 1) * TILE_PIXELS {
            self.player.position.x = (t + 1) * TILE_PIXELS;
        } else if h && self.player.position.x + vx > (u - 1) * TILE_PIXELS {
            self.player.position.x = (u - 1) * TILE_PIXELS;
        } else {
            self.player.position.x += vx;
        }
        
        self.player.velocity.x = 0.0;
        /*
          a  b
        t ┌─┐┌─┐
          | || |
        u └─┘└─┘
            y
        v ┌─┐┌─┐
          | || |
        w └─┘└─┘
        */
        // moving up
        let t = self.player.position.y / TILE_PIXELS - 1;
        let a = self.player.position.x / TILE_PIXELS;
        let b = (self.player.position.x + TILE_PIXELS - 1) / TILE_PIXELS;
        let g = self.map.is_solid(a, t) || self.map.is_solid(b, t);

        // moving down
        let u = (self.player.position.y + TILE_PIXELS) / TILE_PIXELS;
        let h = self.map.is_solid(a, u) || self.map.is_solid(b, u);

        self.player.grounded = h;
        if g && self.player.position.y as f32 + vy < ((t + 1) * TILE_PIXELS) as f32 {
            self.player.position.y = (t + 1) * TILE_PIXELS;
            self.player.velocity.y = 0.0;
        } else if h && (self.player.position.y as f32 + vy) > ((u - 1) * TILE_PIXELS) as f32 {
            self.player.position.y = (u - 1) * TILE_PIXELS;
            self.player.velocity.y = 0.0;
        } else {
            self.player.position.y = (self.player.position.y as f32 + vy) as i32;
            self.player.velocity.y = vy + ACCELERATION.y;
        }
    }

    fn draw(&self) {
        unsafe { *DRAW_COLORS = 0x1213; }
        for y in 0..LEVEL_HEIGHT_SIZE {
            for x in 0..(SCREEN_SIZE as i32 / TILE_PIXELS) {
                if self.map.is_solid(x, y) {
                    draw_crate(TILE_PIXELS * x, TILE_PIXELS * y, 0);
                }
            }
        }

        let flags = if self.player.face_left { BLIT_FLIP_X } else { 0 };
        // blit_sub(BUNNY, self.player_x, self.player_y, 16, 16, animation_frame * 16, 0, BUNNY_WIDTH, flags | BLIT_2BPP);
        let animation_frame = (!self.player.grounded) as usize;
        draw_kangaroo(self.player.position.x as i32, self.player.position.y as i32, animation_frame, flags);

        // // draw lava
        // let camera_bottom = self.camera.height + 160;
        // let lava_y = 160 - (camera_bottom - self.lava.height);
        // let lava_height = camera_bottom - self.lava.height;
        // if lava_y <= 160 {
        //     unsafe { *DRAW_COLORS = 0x22 };
        //     rect(
        //         0,
        //         lava_y,
        //         160,
        //         lava_height as _
        //     );
        // }

        // draw player
        // unsafe { *DRAW_COLORS = 0x40 };
        // blit(PLAYER, self.player_x, self.player_y, 8, 8, BLIT_1BPP);
    }
}

/*
level
player
blocks
goal
player.update()
if player collideswith block adjust position
if player collideswith goal go to next level
*/

// impl Game {
//     const fn init() -> Self {
//         Self {
//             frame: 0,
//             gamepad: 0,
//             previous_gamepad: 0,
//             // pressed_this_frame: 0,
//             face_left: false,
//             camera_top: 0,
//             lava_height: 140,
//             player_x: 40,
//             player_y: 0,
//             player_vy: 0,
//             grounded: false,
//             walking: false,
//         }
//     }

//     fn is_solid(tile_x: i32, tile_y: i32) -> bool {
//         if tile_x == 0 || tile_x == SCREEN_SIZE as i32 / TILE_PIXELS - 1 {
//             true
//         } else {
//             let index = tile_y as usize;
//             if let Some(val) = MAP.get(index) {
//                 val & (1 << (8 - tile_x)) != 0
//             } else {
//                 false
//             }
//         }
//     }

//     // fn would_collide_horizontally(position: i32, velocity: i32) -> bool {
//     //     let tile_x = (position + velocity) / TILE_PIXELS;
//     //     let tile_top = 
//     // }

//     // fn would_collide_vertically(position: i32, velocity: i32) -> bool {

//     // }

//     fn handle_horizontal_collision(&mut self, vx: i32) {
//         let mut tile_x = (self.player_x + vx) / TILE_PIXELS;
//         let tile_top = self.player_y / TILE_PIXELS;
//         let tile_bottom = (self.player_y + (PLAYER_SIZE * TILE_PIXELS) - 1) / TILE_PIXELS;

//         let snap_x = if vx < 0 {
//             (tile_x + 1) * TILE_PIXELS
//         } else {
//             tile_x += PLAYER_SIZE;
//             (tile_x - PLAYER_SIZE) * TILE_PIXELS
//         };

//         if Self::is_solid(tile_x, tile_top) || Self::is_solid(tile_x, tile_bottom) {
//             self.player_x = snap_x;
//         } else {
//             self.player_x += vx;
//         }
//     }

//     /// Handle vertical collisions. Returns true if the player should be stopped.
//     fn handle_vertical_collision(&mut self) -> bool {
//         let mut tile_y = (self.player_y + self.player_vy) / TILE_PIXELS;
//         let tile_left = self.player_x / TILE_PIXELS;
//         let tile_right = (self.player_x + (PLAYER_SIZE * TILE_PIXELS) - 1) / TILE_PIXELS;

//         let snap_y = if self.player_vy < 0 {
//             (tile_y + 1) * TILE_PIXELS
//         } else {
//             tile_y += PLAYER_SIZE;
//             (tile_y  - PLAYER_SIZE) * TILE_PIXELS
//         };

//         if Self::is_solid(tile_left, tile_y) || Self::is_solid(tile_right, tile_y) {
//             self.player_y = snap_y;
//             true
//         } else {
//             self.player_y += self.player_vy as i32;
//             false
//         }
//     }

//     fn update(&mut self) {
//         // update for new frame
//         self.walking = false;
//         self.previous_gamepad = self.gamepad;
//         self.gamepad = unsafe { *GAMEPAD1 };
//         self.frame += 1;

//         // check for horizontal motion

//         // let camera_bottom = self.camera_top + 160;
//         // if self.frame % 10 == 0 {
//         //     self.lava_height += LAVA_SPEED;
//         // }
//         let vx = if self.gamepad & BUTTON_LEFT != 0 {
//             self.face_left = true;
//             self.walking = true;
//             -2
//         } else if self.gamepad & BUTTON_RIGHT != 0 {
//             self.face_left = false;
//             2
//         } else {
//             0
//         };
//         self.handle_horizontal_collision(vx);

//         if self.grounded && (self.gamepad & BUTTON_UP) != 0 {
//             self.player_vy = JUMP_SPEED;
//             self.grounded = false;
//             jump_sound();
//         }
//         if self.handle_vertical_collision() {
//             if self.player_vy > 0 {
//                 self.grounded = true
//             }
//             self.player_vy = 0;
//         } else {
//             self.grounded = false;
//             self.player_vy += ACCELERATION;
//         }

//         // pan up if higher than ...

//         // pan down if lower than ...

//         // check for death
//     }

    // fn draw(&self) {
    //     // let animation_frame = if !self.grounded {
    //     //     1
    //     // } else if self.walking {
    //     //     self.frame / 2 % 3
    //     // } else {
    //     //     0
    //     // };

    //     // draw tiles
    //     // figure out how many to draw
    //     unsafe { *DRAW_COLORS = 0x1213; }
    //     for y in 0..LEVEL_HEIGHT_SIZE {
    //         for x in 0..(SCREEN_SIZE as i32 / TILE_PIXELS) {
    //             if Self::is_solid(x, y) {
    //                 draw_crate(TILE_PIXELS * x, TILE_PIXELS * y, 0);
    //             }
    //         }
    //     }

    //     let flags = if self.face_left { BLIT_FLIP_X } else { 0 };
    //     // blit_sub(BUNNY, self.player_x, self.player_y, 16, 16, animation_frame * 16, 0, BUNNY_WIDTH, flags | BLIT_2BPP);
    //     let animation_frame = (!self.grounded) as usize;
    //     draw_kangaroo(self.player_x, self.player_y, animation_frame, flags);

    //     // draw lava
    //     let camera_bottom = self.camera_top + 160;
    //     let lava_y = 160 - (camera_bottom - self.lava_height);
    //     let lava_height = camera_bottom - self.lava_height;
    //     if lava_y <= 160 {
    //         unsafe { *DRAW_COLORS = 0x22 };
    //         rect(
    //             0,
    //             lava_y,
    //             160,
    //             lava_height as _
    //         );
    //     }

    //     // draw player
    //     // unsafe { *DRAW_COLORS = 0x40 };
    //     // blit(PLAYER, self.player_x, self.player_y, 8, 8, BLIT_1BPP);
    // }
// }

// const CHARACTERS: [Character; 4] = [
//     Character::Kangaroo,
//     Character::Frog,
//     Character::Cricket,
//     Character::Rabbit,
// ];

// static mut GAME: Game = Game::init();

// const MAP: &[u8] = &[1,2,129,2,1,2,1,2,13,2,1,2,97,2,1,2,1,3,255,3];

// const MAP_WIDTH: usize = 16;
// const MAP_HEIGHT: usize = 10;

// fn solid(tile_x: usize, tile_y: usize) -> bool {
//     let index = (tile_y * MAP_WIDTH + tile_x) / 8;
//     let shift = tile_x & 0b111;
//     let result = (MAP[index] >> shift) & 1 == 1;
//     trace(format!("tile_x={tile_x}, tile_y={tile_y}, index={index}, shift={shift}, result={result}"));
//     result
// }

pub(crate) const MY_PALETTE: [u32; 4] = [
    0xadac29,  // yellow-green
    0xb59d5c,  // light brown
    0xac3f09,  // dark brown
    0x000000,  // black
];

#[no_mangle]
unsafe fn start() {
    *PALETTE = MY_PALETTE;
}

static mut LEVEL: Level = Level {
    player: Player {
        gamepad: 0,
        previous_gamepad: 0,
        face_left: false,
        position: XY { x: 32, y: 32 },
        velocity: XY { x: 0.0, y: 0.0 },
        grounded: false,
        walking: false,
        // victory_status: VictoryStatus::Playing,
    },
    map: Map { blocks: [
        0b11111111,
        0b00000000,
        0b01000000,
        0b00100000,
        0b00010000,
        0b00000001,
        0b00000100,
        0b00010000,
        0b00000000,
        0b11111111,
    ] },
    // goal: Goal { position: XY::new(20, 20) },
    // lava: Lava { height: 140 },
    // camera: Camera { height: 0 },
};

//             camera_top: 0,
//             lava_height: 140,
//             player_x: 40,
//             player_y: 0,
//             player_vy: 0,
//             grounded: false,
//             walking: false,

#[no_mangle]
fn update() {
    let level = unsafe { &mut LEVEL };
    level.update();
    level.draw();
    // let game = &mut GAME;
    // game.update();
    // game.draw();
}
