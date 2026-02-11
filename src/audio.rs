use core::arch::asm;

const PIT_CONTROL_PORT: u16 = 0x43;
const PIT_CHANNEL_2: u16 = 0x42;
const SPEAKER_PORT: u16 = 0x61;
const PIT_FREQUENCY: u32 = 1193180;  // Base frequency for PIT

/// Play a beep at a specific frequency for a duration
pub fn beep(frequency_hz: u16, duration_ms: u32) {
    if frequency_hz == 0 {
        return;
    }

    let divisor = (PIT_FREQUENCY / frequency_hz as u32) as u16;

    unsafe {
        // Configure PIT channel 2 for mode 3 (square wave)
        outb(PIT_CONTROL_PORT, 0b10110110);  // 0xB6

        // Set frequency divisor (low byte, then high byte)
        outb(PIT_CHANNEL_2, (divisor & 0xFF) as u8);
        outb(PIT_CHANNEL_2, ((divisor >> 8) & 0xFF) as u8);

        // Enable speaker (set bits 0 and 1)
        let current = inb(SPEAKER_PORT);
        outb(SPEAKER_PORT, current | 0x03);
    }

    // Wait for duration
    spin_wait_ms(duration_ms);

    // Disable speaker (clear bits 0 and 1)
    unsafe {
        let current = inb(SPEAKER_PORT);
        outb(SPEAKER_PORT, current & 0xFC);
    }
}

/// Play a simple melody (array of notes)
pub fn play_melody(notes: &[(u16, u32)]) {
    for &(frequency, duration) in notes {
        beep(frequency, duration);
    }
}

/// Spin-wait for milliseconds using the PIT
fn spin_wait_ms(ms: u32) {
    // Simple busy-wait loop
    // This is a rough approximation - adjust the constant as needed
    for _ in 0..ms {
        for _ in 0..1000 {
            unsafe {
                asm!("nop");
            }
        }
    }
}

/// Output a byte to an I/O port
unsafe fn outb(port: u16, value: u8) {
    asm!(
        "out dx, al",
        in("dx") port,
        in("al") value,
        options(nostack)
    );
}

/// Read a byte from an I/O port
unsafe fn inb(port: u16) -> u8 {
    let value: u8;
    asm!(
        "in al, dx",
        in("dx") port,
        out("al") value,
        options(nostack)
    );
    value
}

// Some common frequencies for musical notes
pub struct Notes;

impl Notes {
    pub const C4: u16 = 262;   // Middle C
    pub const D4: u16 = 294;
    pub const E4: u16 = 330;
    pub const F4: u16 = 349;
    pub const G4: u16 = 392;
    pub const A4: u16 = 440;
    pub const B4: u16 = 494;
    pub const C5: u16 = 523;
}
