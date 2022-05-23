use crate::{DRAW_COLORS, GAMEPAD1, PALETTE};

pub fn set_palette() {
    unsafe { *PALETTE = [0xf08c48, 0xe95f37, 0xbc593d, 0x805043] };
}

pub fn set_draw_colors(color: u16) {
    unsafe { *DRAW_COLORS = color };
}

pub fn get_gamepad() -> u8 {
    unsafe { *GAMEPAD1 }
}
