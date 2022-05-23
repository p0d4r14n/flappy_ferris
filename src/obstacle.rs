use crate::{player::*, position::*, rect, set_draw_colors, SCREEN_SIZE, SPRITE_SIZE};

pub struct Obstacle {
    pub position: Position,
    size: u32,
    pub width: u32,
}

impl Obstacle {
    pub const fn new(position: Position, size: u32, width: u32) -> Self {
        Obstacle {
            position,
            size,
            width,
        }
    }

    pub fn update(&mut self) {
        self.position.x -= 0.4;
    }

    pub fn render(&self) {
        set_draw_colors(0x43);
        rect(
            self.position.x as i32,
            -1,
            self.width,
            (self.position.y as u32) - self.size / 2,
        );
        rect(
            self.position.x as i32,
            self.position.y as i32 + (self.size / 2) as i32,
            self.width,
            SCREEN_SIZE - (self.position.y as u32 + self.size / 2) + 1,
        );
        set_draw_colors(0x4321);
    }

    pub fn hit(&self, player: &Player) -> bool {
        // TODO: eliminate magic numbers and simplify types
        (player.position.x + 10.0 < self.position.x + self.width as f32
            && player.position.x as i32 + (SPRITE_SIZE as i32 - 10) > self.position.x as i32)
            && ((player.position.y as i32 + 20) < self.position.y as i32 - (self.size / 2) as i32
                || (player.position.y as i32) + (SPRITE_SIZE as i32 - 20)
                    > self.position.y as i32 + self.position.y as i32 / 2)
    }
}
