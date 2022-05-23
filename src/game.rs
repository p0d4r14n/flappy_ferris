use crate::{
    get_gamepad, obstacle::*, player::*, position::*, text, BUTTON_1, SCREEN_SIZE, SPRITE_SIZE,
};

pub enum State {
    Menu,
    Running,
    GameOver,
}

pub struct Game {
    player: Player,
    obstacle: Obstacle,
    score: i32,
    prev_input: u8,
    pub state: State,
}

impl Game {
    pub const fn new() -> Self {
        Game {
            player: Player::new(Position {
                x: 10.0,
                y: (SCREEN_SIZE / 2 - SPRITE_SIZE / 2) as f32,
            }),
            obstacle: Obstacle::new(
                Position {
                    x: (SCREEN_SIZE - 10) as f32,
                    y: (SCREEN_SIZE / 2) as f32,
                },
                100,
                10,
            ),
            score: 0,
            prev_input: 0,
            state: State::Menu,
        }
    }

    pub fn restart(&mut self) {
        self.player = Player::new(Position {
            x: 10.0,
            y: (SCREEN_SIZE / 2 - SPRITE_SIZE / 2) as f32,
        });
        self.obstacle = Obstacle::new(
            Position {
                x: (SCREEN_SIZE - 10) as f32,
                y: (SCREEN_SIZE / 2) as f32,
            },
            100,
            10,
        );
        self.score = 0;
    }

    pub fn update(&mut self) {
        self.player.update();
        self.obstacle.update();

        if self.obstacle.hit(&self.player) {
            self.state = State::GameOver;
        }

        if self.obstacle.position.x < 0.0 - self.obstacle.width as f32 {
            self.score += 1;
            self.obstacle = Obstacle::new(
                Position {
                    x: SCREEN_SIZE as f32,
                    y: (SCREEN_SIZE / 2) as f32,
                },
                100,
                10,
            );
        }
    }

    pub fn input(&mut self) {
        let gamepad: u8 = get_gamepad();
        let just_pressed: u8 = gamepad & (gamepad ^ self.prev_input);

        match self.state {
            State::Running => {
                if just_pressed != 0 && BUTTON_1 != 0 {
                    self.player.jump();
                }
            }
            State::Menu => {
                if just_pressed != 0 && BUTTON_1 != 0 {
                    self.state = State::Running;
                }
                // TODO: implement quit
            }
            State::GameOver => {
                self.restart();
                if just_pressed != 0 && BUTTON_1 != 0 {
                    self.state = State::Running;
                }
                // TODO: implement quit
            }
        }

        self.prev_input = gamepad;
    }

    pub fn render(&self) {
        self.player.render();
        self.obstacle.render();
    }

    pub fn render_menu(&self) {
        text("Flappy Ferris", 10, 10);
        text("(X) Start Game", 10, 30);
        text("(Z) Quit Game", 10, 40);
    }

    pub fn render_game_over(&self) {
        text("Game Over", 10, 10);
        text("(X) Restart Game", 10, 30);
        text("(Z) Quit Game", 10, 40);
    }
}
