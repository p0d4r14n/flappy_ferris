use crate::{blit, position::*, FERRIS_FALL, FERRIS_JUMP, SCREEN_SIZE, SPRITE_SIZE};

pub struct Player {
    pub position: Position,
    velocity: f32,
}

impl Player {
    pub const fn new(position: Position) -> Self {
        Player {
            position,
            velocity: 0.0,
        }
    }

    pub fn update(&mut self) {
        if self.velocity < 1.0 {
            self.velocity += 0.1;
        }

        self.position.y += self.velocity;
        if self.position.y < 0.0 {
            self.position.y = 0.0;
        } else if self.position.y > (SCREEN_SIZE - SPRITE_SIZE) as f32 {
            self.position.y = (SCREEN_SIZE - SPRITE_SIZE) as f32;
        }
    }

    pub fn render(&self) {
        if self.velocity > 0.0 {
            blit(
                &FERRIS_FALL,
                self.position.x as i32,
                self.position.y as i32,
                SPRITE_SIZE,
                SPRITE_SIZE,
                1,
            );
        } else {
            blit(
                &FERRIS_JUMP,
                self.position.x as i32,
                self.position.y as i32,
                SPRITE_SIZE,
                SPRITE_SIZE,
                1,
            );
        }
    }

    pub fn jump(&mut self) {
        self.velocity = -2.0;
    }
}
