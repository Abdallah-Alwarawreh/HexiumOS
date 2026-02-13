const VGA_GRAPHICS_BUFFER: usize = 0xa0000;
const SCREEN_WIDTH: usize = 320;
const SCREEN_HEIGHT: usize = 200;

pub struct graphics;

use core::arch::asm;

use crate::io::outb; // Import the public function

use font::FONT_8X8;


// Helper for reading from ports
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!("in al, dx", out("al") value, in("dx") port, options(nomem, nostack));
    value
}


impl graphics {
    /// Draws a pixel at (x, y) with a specific color index (0-255)
    pub fn draw_pixel(&self, x: usize, y: usize, color: u8) {
        if x >= SCREEN_WIDTH || y >= SCREEN_HEIGHT {
            return; // Stay within bounds to avoid memory corruption
        }

        // The formula for the memory offset in Mode 13h
        let offset = y * SCREEN_WIDTH + x;

        unsafe {
            let pixel_ptr = VGA_GRAPHICS_BUFFER as *mut u8;
            *pixel_ptr.add(offset) = color;
        }
    }
    
pub fn draw_char(&self, c: char, x: usize, y: usize, color: u8) {
    let mut index = c as usize;
    index -= 0;
    if index >= FONT_8X8.len() { return; } // Safety check

    let glyph = FONT_8X8[index];
    for (row, row_data) in glyph.iter().enumerate() {
        for col in 0..8 {
            if (row_data >> (7 - col)) & 1 == 1 {
                self.draw_pixel(x + col, y + row, color);
            }
        }
    }
}

    pub fn clear_screen(&self, color: u8) {
        for y in 0..SCREEN_HEIGHT {
            for x in 0..SCREEN_WIDTH {
                self.draw_pixel(x, y, color);
            }
        }
    }
    
pub unsafe fn enter_mode_13h() {
        // 1. MISC Output
        outb(0x3C2, 0x63);

        // 2. Sequencer Registers
        let seq: [u8; 5] = [0x03, 0x01, 0x0F, 0x00, 0x0E]; // 0x0E here is the "Chain-4" magic
        for i in 0..5 {
            outb(0x3C4, i as u8);
            outb(0x3C5, seq[i]);
        }

        // 3. CRTC Registers (Unlock first)
        outb(0x3D4, 0x03); outb(0x3D5, inb(0x3D5) | 0x80);
        outb(0x3D4, 0x11); outb(0x3D5, inb(0x3D5) & !0x80);

        let crtc: [u8; 25] = [
            0x5F, 0x4F, 0x50, 0x82, 0x54, 0x80, 0xBF, 0x1F,
            0x00, 0x41, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
            0x9C, 0x0E, 0x8F, 0x28, 0x40, 0x96, 0xB9, 0xA3,
            0xFF
        ];
        for i in 0..25 {
            outb(0x3D4, i as u8);
            outb(0x3D5, crtc[i]);
        }

        // 4. Graphics Controller
        let gc: [u8; 9] = [0x00, 0x00, 0x00, 0x00, 0x00, 0x40, 0x05, 0x0F, 0xFF];
        for i in 0..9 {
            outb(0x3CE, i as u8);
            outb(0x3CF, gc[i]);
        }

        // 5. Attribute Controller (Special toggle logic)
        inb(0x3DA); // Reset flip-flop
        for i in 0..16 {
            outb(0x3C0, i as u8);
            outb(0x3C0, i as u8); // Palette
        }
        let atc: [u8; 5] = [0x41, 0x00, 0x0F, 0x00, 0x00];
        for i in 0..5 {
            outb(0x3C0, (i + 16) as u8);
            outb(0x3C0, atc[i]);
        }
        outb(0x3C0, 0x20); // Enable display
    }
}
