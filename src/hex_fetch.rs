use crate::vga_colors::Color;
use crate::writer::Writer;
use core::arch::asm;

pub struct HexFetch {}

struct CpuInfo {
    vendor: [u8; 12],
    brand: [u8; 48],
    has_brand: bool,
}

impl CpuInfo {
    fn detect() -> Self {
        let mut info = CpuInfo {
            vendor: [0; 12],
            brand: [0; 48],
            has_brand: false,
        };

        let (max_func, ebx, ecx, edx) = cpuid(0);
        info.vendor[0..4].copy_from_slice(&ebx.to_le_bytes());
        info.vendor[4..8].copy_from_slice(&edx.to_le_bytes());
        info.vendor[8..12].copy_from_slice(&ecx.to_le_bytes());

        let (max_ext, _, _, _) = cpuid(0x80000000);
        if max_ext >= 0x80000004 {
            info.has_brand = true;
            for i in 0..3 {
                let (eax, ebx, ecx, edx) = cpuid(0x80000002 + i);
                let offset = (i as usize) * 16;
                info.brand[offset..offset + 4].copy_from_slice(&eax.to_le_bytes());
                info.brand[offset + 4..offset + 8].copy_from_slice(&ebx.to_le_bytes());
                info.brand[offset + 8..offset + 12].copy_from_slice(&ecx.to_le_bytes());
                info.brand[offset + 12..offset + 16].copy_from_slice(&edx.to_le_bytes());
            }
        }

        info
    }

    fn vendor_str(&self) -> &str {
        core::str::from_utf8(&self.vendor).unwrap_or("Unknown")
    }

    fn brand_str(&self) -> &str {
        if self.has_brand {
            let s = core::str::from_utf8(&self.brand).unwrap_or("Unknown");
            s.trim_matches(|c: char| c == '\0' || c == ' ')
        } else {
            self.vendor_str()
        }
    }
}

fn cpuid(function: u32) -> (u32, u32, u32, u32) {
    let eax: u32;
    let ebx: u32;
    let ecx: u32;
    let edx: u32;

    unsafe {
        asm!(
            "cpuid",
            inout("eax") function => eax,
            out("ebx") ebx,
            out("ecx") ecx,
            out("edx") edx,
            options(nostack, preserves_flags)
        );
    }

    (eax, ebx, ecx, edx)
}

fn detect_memory_kb() -> u32 {
    let base_mem: u16 = unsafe { *(0x413 as *const u16) };

    let extended_kb: u32 = 128 * 1024;

    (base_mem as u32) + extended_kb
}

fn get_uptime_seconds() -> u32 {
    let ticks: u32 = unsafe { *(0x46C as *const u32) };
    ticks / 18
}

impl HexFetch {
    pub fn fetch(writer: &mut Writer) {
        let cpu = CpuInfo::detect();
        let memory_kb = detect_memory_kb();
        let memory_mb = memory_kb / 1024;
        let uptime = get_uptime_seconds();

        let hours = uptime / 3600;
        let minutes = (uptime % 3600) / 60;
        let seconds = uptime % 60;

        // Smaller ASCII art (30 chars wide) + info on right
        // Line 1
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str("    __  __          _            ");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("OS: ");
        writer.set_color(Color::White, Color::Black);
        writer.write_str("HexiumOS\n");

        // Line 2
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str("   / / / /__  _  __(_)_  ______ _");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("Kernel: ");
        writer.set_color(Color::White, Color::Black);
        writer.write_str("0.1.0\n");

        // Line 3
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str("  / /_/ / _ \\| |/_/ / / / / __ `/");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("Uptime: ");
        writer.set_color(Color::White, Color::Black);
        write_uptime(writer, hours, minutes, seconds);
        writer.write_str("\n");

        // Line 4
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str(" / __  /  __/>  </ / /_/ / /_/ / ");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("Shell: ");
        writer.set_color(Color::White, Color::Black);
        writer.write_str("HexShell\n");

        // Line 5
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str("/_/ /_/\\___/_/|_/_/\\__,_/\\__,_/  ");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("CPU: ");
        writer.set_color(Color::White, Color::Black);
        // Truncate CPU name to fit
        write_truncated(writer, cpu.brand_str(), 25);
        writer.write_str("\n");

        // Line 6 - Memory
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str("                                 ");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("Memory: ");
        writer.set_color(Color::White, Color::Black);
        write_number(writer, memory_mb);
        writer.write_str(" MB\n");

        // Line 7 - Arch
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_str("                                 ");
        writer.set_color(Color::Yellow, Color::Black);
        writer.write_str("Arch: ");
        writer.set_color(Color::White, Color::Black);
        writer.write_str("i386\n");

        // Color palette display
        writer.write_str("\n    ");
        for i in 0..8 {
            let color = match i {
                0 => Color::Black,
                1 => Color::Red,
                2 => Color::Green,
                3 => Color::Brown,
                4 => Color::Blue,
                5 => Color::Magenta,
                6 => Color::Cyan,
                7 => Color::LightGray,
                _ => Color::Black,
            };
            writer.set_color(color, color);
            writer.write_str("  ");
        }
        writer.set_color(Color::White, Color::Black);
        writer.write_str("\n    ");
        for i in 0..8 {
            let color = match i {
                0 => Color::DarkGray,
                1 => Color::LightRed,
                2 => Color::LightGreen,
                3 => Color::Yellow,
                4 => Color::LightBlue,
                5 => Color::Pink,
                6 => Color::LightCyan,
                7 => Color::White,
                _ => Color::Black,
            };
            writer.set_color(color, color);
            writer.write_str("  ");
        }
        writer.set_color(Color::White, Color::Black);
        writer.write_str("\n");
    }
}

/// Write a string truncated to max_len characters
fn write_truncated(writer: &mut Writer, s: &str, max_len: usize) {
    let bytes = s.as_bytes();
    if bytes.len() <= max_len {
        writer.write_str(s);
    } else {
        for i in 0..max_len {
            writer.write_byte(bytes[i]);
        }
    }
}

fn write_number(writer: &mut Writer, mut n: u32) {
    if n == 0 {
        writer.write_str("0");
        return;
    }

    let mut buf = [0u8; 10];
    let mut i = 0;

    while n > 0 {
        buf[i] = b'0' + (n % 10) as u8;
        n /= 10;
        i += 1;
    }

    while i > 0 {
        i -= 1;
        writer.write_byte(buf[i]);
    }
}

fn write_uptime(writer: &mut Writer, hours: u32, minutes: u32, seconds: u32) {
    if hours > 0 {
        write_number(writer, hours);
        writer.write_str("h ");
    }
    write_number(writer, minutes);
    writer.write_str("m ");
    write_number(writer, seconds);
    writer.write_str("s");
}
