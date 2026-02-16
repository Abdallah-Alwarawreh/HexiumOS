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
pub mod io;

use vga_colors::{Color, color_code};
use writer::Writer;
use cli::CLI;

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    let mut writer = Writer::new(color_code(Color::Red, Color::Black));
    writer.clear();
    writer.set_color(Color::Red, Color::Black);
    writer.write_str("[KERNEL PANIC]\n");
    if let Some(location) = info.location() {
        writer.write_str("Panic at ");
        writer.write_str(location.file());
        writer.write_str(":");
        writer.write_u64(location.line() as u64);
        writer.write_str("\n");
    }
    if let Some(message) = info.message() {
        use core::fmt::Write;
        let _ = write!(writer, "Message: {}\n", message);
    }
    writer.write_str("System halted.");
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer::new(color_code(Color::White, Color::Black));
    writer.clear();
    writer.enable_cursor();

    idt::init();

    writer.set_color(Color::LightCyan, Color::Black);
    writer.write_str(
        "  _    _           _                  ____   _____ \n\
         | |  | |         (_)                / __ \\ / ____|\n\
         | |__| | _____  ___ _   _ _ __ ___ | |  | | (___  \n\
         |  __  |/ _ \\ \\/ / | | | | '_ ` _ \\| |  | |\\___ \\ \n\
         | |  | |  __/>  <| | |_| | | | | | | |__| |____) |\n\
         |_|  |_|\\___/_/\\_\\_|\\__,_|_| |_| |_|\\____/|_____/ \n"
    );
    writer.write_str("\n");

    writer.set_color(Color::LightCyan, Color::Black);
    writer.write_str("=== Welcome to HexiumOS ===\n");
    writer.set_color(Color::White, Color::Black);
    writer.write_str("Type 'help' for available commands.\n\n");

    let fs = filesystem::get_filesystem();
    fs.init();

    writer.set_color(Color::Yellow, Color::Black);
    writer.write_str("Loaded modules: ");
    writer.write_str("vga_colors, writer, keyboard, cli, intrinsics, idt, snake, video_player, bad_apple_data, RAHH_data, filesystem, editor, hex_fetch, graphics, io\n");

    writer.set_color(Color::Green, Color::Black);
    writer.write_str("Running POST... ");
    if fs.test_integrity() {
        writer.write_str("OK\n");
    } else {
        writer.write_str("FAIL\n");
    }

    writer.set_color(Color::Magenta, Color::Black);
    writer.write_str("Kernel time: ");
    let uptime = intrinsics::get_uptime();
    writer.write_u64(uptime);
    writer.write_str(" ticks\n");

    writer.set_color(Color::White, Color::Black);

    let mut cli = CLI::new();
    cli.run(&mut writer);
}
