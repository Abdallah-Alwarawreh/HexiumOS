#![no_std]
#![no_main]
#![feature(naked_functions)]

use core::panic::PanicInfo;

mod vga_colors;
mod writer;
mod io;
mod pic;
mod idt;
mod keyboard;
mod interrupts;

use vga_colors::{Color, color_code};
use writer::Writer;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static mut WRITER: Option<Writer> = None;

fn get_writer() -> &'static mut Writer {
    unsafe {
        WRITER.as_mut().unwrap()
    }
}

#[no_mangle]
pub extern "C" fn kernel_main() -> ! {
    unsafe {
        WRITER = Some(Writer::new(color_code(Color::LightGreen, Color::Black)));
    }
    
    let writer = get_writer();
    writer.clear();

    writer.write_str("Welcome to Hexium OS!\n");
    writer.write_str("> ");

    unsafe {
        interrupts::init();
    }

    loop {
        if keyboard::key_available() {
            if let Some(key) = keyboard::pop_key() {
                let writer = get_writer();
                match key {
                    keyboard::KEY_BACKSPACE => {
                        writer.backspace();
                    }
                    keyboard::KEY_ENTER => {
                        writer.write_byte(b'\n');
                        writer.write_str("> ");
                    }
                    _ => {
                        writer.write_byte(key);
                    }
                }
            }
        }
        
        unsafe {
            core::arch::asm!("hlt");
        }
    }
}