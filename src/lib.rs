#![no_std]
mod game;
mod obstacle;
mod player;
mod position;
mod sprite;
mod utils;
mod wasm4;

use core::panic::PanicInfo;
use game::*;
use sprite::*;
use utils::*;
use wasm4::*;

#[panic_handler]
fn panic(_panic_info: &PanicInfo) -> ! {
    trace("Panic!");
    loop {}
}

pub static mut GAME: Game = Game::new();

#[no_mangle]
fn start() {
    set_palette();
    set_draw_colors(0x4321);
}

#[no_mangle]
fn update() {
    let game = unsafe { &mut GAME };
    match game.state {
        State::Running => {
            game.update();
            game.render();
        }
        State::Menu => game.render_menu(),
        State::GameOver => game.render_game_over(),
    }
    game.input();
}
