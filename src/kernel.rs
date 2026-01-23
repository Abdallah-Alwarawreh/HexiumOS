#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod vga_colors;
mod writer;

use vga_colors::{Color, color_code};
use writer::Writer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    let mut writer = Writer::new(color_code(Color::LightGreen, Color::Black));
    writer.clear();

    writer.write_str("Welcome to Hexium OS!\n");

    loop {}
}


