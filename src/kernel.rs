#![no_std]
#![no_main]
#![feature(lang_items)]

use core::panic::PanicInfo;

mod vga_colors;
mod writer;
mod keyboard;
mod cli;
mod intrinsics;
mod idt;
mod snake;
mod video_player;
mod bad_apple_data;
mod RAHH_data;
mod filesystem;
mod editor;

mod hex_fetch;

mod graphics;

mod snake_graphics;

pub mod io;

mod font;

use vga_colors::{Color, color_code};
use writer::Writer;
use cli::CLI;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer::new(color_code(Color::White, Color::Black));
    writer.clear();
    writer.enable_cursor();

    idt::init();

    writer.set_color(Color::LightCyan, Color::Black);
    writer.write_str("  _    _           _                  ____   _____ \n");
    writer.write_str(" | |  | |         (_)                / __ \\ / ____|\n");
    writer.write_str(" | |__| | _____  ___ _   _ _ __ ___ | |  | | (___  \n");
    writer.write_str(" |  __  |/ _ \\ \\/ / | | | | '_ ` _ \\| |  | |\\___ \\ \n");
    writer.write_str(" | |  | |  __/>  <| | |_| | | | | | | |__| |____) |\n");
    writer.write_str(" |_|  |_|\\___/_/\\_\\_|\\__,_|_| |_| |_|\\____/|_____/ \n");
    writer.write_str("                                               \n");
    writer.write_str("                                               \n");

    writer.set_color(Color::LightCyan, Color::Black);
    writer.write_str("=== Welcome to HexiumOS ===\n");
    writer.set_color(Color::White, Color::Black);
    writer.write_str("Type 'help' for available commands.\n\n");

    filesystem::get_filesystem().init();

    let mut cli = CLI::new();
    cli.run(&mut writer);
}


