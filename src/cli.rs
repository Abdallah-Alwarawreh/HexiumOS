use crate::writer::Writer;
use crate::keyboard::Keyboard;
use crate::vga_colors::Color;
use crate::idt;
use crate::snake::SnakeGame;
use crate::video_player::VideoPlayer;
use crate::bad_apple_data::{FRAME_COUNT as BAD_APPLE_FRAME_COUNT, FRAME_WIDTH as BAD_APPLE_FRAME_WIDTH, FRAME_HEIGHT as BAD_APPLE_FRAME_HEIGHT, TARGET_FPS as BAD_APPLE_TARGET_FPS, FRAMES as BAD_APPLE_FRAMES};
use crate::RAHH_data::{FRAME_COUNT as RAHH_FRAME_COUNT, FRAME_WIDTH as RAHH_FRAME_WIDTH, FRAME_HEIGHT as RAHH_FRAME_HEIGHT, TARGET_FPS as RAHH_TARGET_FPS, FRAMES as RAHH_FRAMES};
use crate::filesystem::{get_filesystem, FileEntry};
use crate::editor::Editor;


use crate::hex_fetch::HexFetch;

use crate::graphics::graphics;

const MAX_COMMAND_LEN: usize = 80;

pub struct CLI {
    buffer: [u8; MAX_COMMAND_LEN],
    buffer_len: usize,
    shift_pressed: bool,
}

impl CLI {
    pub const fn new() -> Self {
        Self {
            buffer: [0; MAX_COMMAND_LEN],
            buffer_len: 0,
            shift_pressed: false,
        }
    }

    pub fn show_prompt(&self, writer: &mut Writer) {
        writer.set_color(Color::LightGreen, Color::Black);
        writer.write_str("HexiumOS> ");
        writer.set_color(Color::White, Color::Black);
    }

    pub fn run(&mut self, writer: &mut Writer) -> ! {
        self.show_prompt(writer);

        loop {
            let scancode = match idt::get_scancode() {
                Some(sc) => sc,
                None => {
                    idt::wait_for_interrupt();
                    continue;
                }
            };

            if scancode & 0x80 != 0 {
                if scancode == 0xAA || scancode == 0xB6 {
                    self.shift_pressed = false;
                }
                continue;
            }

            if scancode == 0x2A || scancode == 0x36 {
                self.shift_pressed = true;
                continue;
            }

            if scancode == 0x1C {
                writer.write_byte(b'\n');
                self.execute_command(writer);
                self.buffer_len = 0;
                self.show_prompt(writer);
                continue;
            }

            if scancode == 0x0E {
                if self.buffer_len > 0 {
                    self.buffer_len -= 1;
                    self.delete_char(writer);
                }
                continue;
            }

            if let Some(c) = Keyboard::scancode_to_char(scancode, self.shift_pressed) {
                if self.buffer_len < MAX_COMMAND_LEN {
                    self.buffer[self.buffer_len] = c as u8;
                    self.buffer_len += 1;
                    writer.write_byte(c as u8);
                }
            }
        }
    }

    fn delete_char(&self, writer: &mut Writer) {
        unsafe {
            let vga = 0xb8000 as *mut u8;
            if writer.get_col() > 0 {
                let col = writer.get_col() - 1;
                let row = writer.get_row();
                let offset = (row * 80 + col) * 2;
                *vga.add(offset) = b' ';
                writer.set_position(col, row);
            }
        }
    }

    fn execute_command(&self, writer: &mut Writer) {
        if self.buffer_len == 0 {
            return;
        }

        let cmd = &self.buffer[..self.buffer_len];

        if cmd == b"help" {
            writer.set_color(Color::LightCyan, Color::Black);
            writer.write_str("Available commands:\n");
            writer.set_color(Color::White, Color::Black);
            writer.write_str("  help          - Show this help message\n");
            writer.write_str("  clear         - Clear the screen\n");
            writer.write_str("  hello         - Print a greeting\n");
            writer.write_str("  info          - Display system information\n");
            writer.write_str("  echo <text>   - Echo back the text\n");
            writer.write_str("  snake         - Play the snake game\n");
            writer.write_str("  play <video>  - Play a video (badapple)\n");
            writer.set_color(Color::LightCyan, Color::Black);
            writer.write_str("File System:\n");
            writer.set_color(Color::White, Color::Black);
            writer.write_str("  ls            - List files and directories\n");
            writer.write_str("  cat <file>    - Display file contents\n");
            writer.write_str("  edit <file>   - Edit a file\n");
            writer.write_str("  touch <file>  - Create an empty file\n");
            writer.write_str("  write <file>  - Write text to file\n");
            writer.write_str("  rm <file>     - Delete a file\n");
            writer.write_str("  mkdir <dir>   - Create a directory\n");
            writer.write_str("  rmdir <dir>   - Remove a directory\n");
            writer.write_str("  cd <dir>      - Change directory\n");
            writer.write_str("  pwd           - Print working directory\n");
        } else if cmd == b"clear" {
            writer.clear();
        } else if cmd == b"hello" {
            writer.set_color(Color::Yellow, Color::Black);
            writer.write_str("Hello from HexiumOS!\n");
            writer.set_color(Color::White, Color::Black);
        } else if cmd == b"info" {
            writer.set_color(Color::LightCyan, Color::Black);
            writer.write_str("=== Hexium HexiumOS ===\n");
            writer.set_color(Color::White, Color::Black);
            writer.write_str("A simple operating system written in Rust\n");
            writer.write_str("Version: 0.1.0\n");
        } else if cmd.starts_with(b"echo ") {
            writer.write_bytes(&cmd[5..]);
            writer.write_byte(b'\n');
        } else if cmd.starts_with(b"play ") {
            let video_name = &cmd[5..];
            if video_name == b"badapple" {
                let mut player = VideoPlayer::new(BAD_APPLE_FRAMES, BAD_APPLE_FRAME_COUNT, BAD_APPLE_FRAME_WIDTH, BAD_APPLE_FRAME_HEIGHT, BAD_APPLE_TARGET_FPS);
                player.run();
                writer.clear();
                writer.write_str("Video finished!\n");
            } else if video_name == b"RAHH" {
                let mut player = VideoPlayer::new(RAHH_FRAMES, RAHH_FRAME_COUNT, RAHH_FRAME_WIDTH, RAHH_FRAME_HEIGHT, RAHH_TARGET_FPS);
                player.run();
                writer.clear();
                writer.write_str("Video finished!\n");
            } else {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Unknown video: ");
                writer.write_bytes(video_name);
                writer.write_str("\nAvailable videos: badapple\n");
                writer.set_color(Color::White, Color::Black);
            }
        } else if cmd == b"snake" {
            let mut game = SnakeGame::new();
            game.run(writer);
            writer.clear();
            writer.write_str("Thanks for playing!\n");
        } else if cmd == b"ls" {
            self.cmd_ls(writer);
        } else if cmd.starts_with(b"cat ") {
            self.cmd_cat(&cmd[4..], writer);
        } else if cmd.starts_with(b"edit ") {
            self.cmd_edit(&cmd[5..], writer);
        } else if cmd.starts_with(b"touch ") {
            self.cmd_touch(&cmd[6..], writer);
        } else if cmd.starts_with(b"write ") {
            self.cmd_write(&cmd[6..], writer);
        } else if cmd.starts_with(b"rm ") {
            self.cmd_rm(&cmd[3..], writer);
        } else if cmd.starts_with(b"mkdir ") {
            self.cmd_mkdir(&cmd[6..], writer);
        } else if cmd.starts_with(b"rmdir ") {
            self.cmd_rmdir(&cmd[6..], writer);
        } else if cmd.starts_with(b"cd ") {
            self.cmd_cd(&cmd[3..], writer);
        } else if cmd == b"pwd" {
            self.cmd_pwd(writer);
        } else if cmd == b"hexfetch" {
           HexFetch::fetch(writer);
        } else if cmd == b"graphictest" {

            unsafe {
                    graphics::enter_mode_13h(); 
        
                    let graphics_test = graphics; 
                    graphics_test.clear_screen(0); 
                    for x in 0..320 {
                        graphics_test.draw_pixel(x, 0, 15); // White line at y=0
                    }
                    graphics_test.draw_pixel(160, 100, 14); // Draw a yellow pixel in the center
                    
                    for y in 0..50 {
                        for x in 0..50 {
                         graphics_test.draw_pixel(x, y, 14);
                       }
                       }
        
                    loop {
                        if idt::get_scancode().is_some() { break; }
                    }
                }
            
        } else {
            writer.set_color(Color::Red, Color::Black);
            writer.write_str("Unknown command: ");
            writer.write_bytes(cmd);
            writer.write_str("\nType 'help' for available commands.\n");
            writer.set_color(Color::White, Color::Black);
        }
    }

    fn cmd_ls(&self, writer: &mut Writer) {
        let fs = get_filesystem();
        let mut has_entries = false;
        
        for entry in fs.list_files() {
            has_entries = true;
            match entry {
                FileEntry::Directory(name) => {
                    writer.set_color(Color::LightBlue, Color::Black);
                    writer.write_bytes(name);
                    writer.write_str("/\n");
                }
                FileEntry::File(name, size) => {
                    writer.set_color(Color::White, Color::Black);
                    writer.write_bytes(name);
                    writer.write_str("  (");
                    self.write_number(writer, size);
                    writer.write_str(" bytes)\n");
                }
            }
        }
        
        if !has_entries {
            writer.set_color(Color::DarkGray, Color::Black);
            writer.write_str("(empty directory)\n");
        }
        writer.set_color(Color::White, Color::Black);
    }

    fn cmd_cat(&self, filename: &[u8], writer: &mut Writer) {
        let fs = get_filesystem();
        match fs.read_file(filename) {
            Some(content) => {
                writer.write_bytes(content);
                if !content.is_empty() && content[content.len() - 1] != b'\n' {
                    writer.write_byte(b'\n');
                }
            }
            None => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("File not found: ");
                writer.write_bytes(filename);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_edit(&self, filename: &[u8], writer: &mut Writer) {
        let mut editor = Editor::new();
        match editor.open(filename) {
            Ok(()) => {
                editor.run(writer);
                writer.clear();
            }
            Err(e) => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Error: ");
                writer.write_str(e);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_touch(&self, filename: &[u8], writer: &mut Writer) {
        let fs = get_filesystem();
        if fs.file_exists(filename) {
            writer.write_str("File already exists\n");
            return;
        }
        match fs.create_file(filename, b"") {
            Ok(()) => {
                writer.set_color(Color::Green, Color::Black);
                writer.write_str("Created: ");
                writer.write_bytes(filename);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
            Err(e) => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Error: ");
                writer.write_str(e);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_write(&self, args: &[u8], writer: &mut Writer) {
        let mut space_idx = None;
        for (i, &b) in args.iter().enumerate() {
            if b == b' ' {
                space_idx = Some(i);
                break;
            }
        }

        match space_idx {
            Some(idx) => {
                let filename = &args[..idx];
                let content = &args[idx + 1..];
                
                let fs = get_filesystem();
                match fs.write_file(filename, content) {
                    Ok(()) => {
                        writer.set_color(Color::Green, Color::Black);
                        writer.write_str("Written to: ");
                        writer.write_bytes(filename);
                        writer.write_byte(b'\n');
                        writer.set_color(Color::White, Color::Black);
                    }
                    Err(e) => {
                        writer.set_color(Color::Red, Color::Black);
                        writer.write_str("Error: ");
                        writer.write_str(e);
                        writer.write_byte(b'\n');
                        writer.set_color(Color::White, Color::Black);
                    }
                }
            }
            None => {
                writer.set_color(Color::Yellow, Color::Black);
                writer.write_str("Usage: write <filename> <content>\n");
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_rm(&self, filename: &[u8], writer: &mut Writer) {
        let fs = get_filesystem();
        match fs.delete_file(filename) {
            Ok(()) => {
                writer.set_color(Color::Green, Color::Black);
                writer.write_str("Deleted: ");
                writer.write_bytes(filename);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
            Err(e) => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Error: ");
                writer.write_str(e);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_mkdir(&self, dirname: &[u8], writer: &mut Writer) {
        let fs = get_filesystem();
        match fs.create_directory(dirname) {
            Ok(()) => {
                writer.set_color(Color::Green, Color::Black);
                writer.write_str("Created directory: ");
                writer.write_bytes(dirname);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
            Err(e) => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Error: ");
                writer.write_str(e);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_rmdir(&self, dirname: &[u8], writer: &mut Writer) {
        let fs = get_filesystem();
        match fs.remove_directory(dirname) {
            Ok(()) => {
                writer.set_color(Color::Green, Color::Black);
                writer.write_str("Removed directory: ");
                writer.write_bytes(dirname);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
            Err(e) => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Error: ");
                writer.write_str(e);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_cd(&self, dirname: &[u8], writer: &mut Writer) {
        let fs = get_filesystem();
        match fs.change_directory(dirname) {
            Ok(()) => {
                // Success
            }
            Err(e) => {
                writer.set_color(Color::Red, Color::Black);
                writer.write_str("Error: ");
                writer.write_str(e);
                writer.write_byte(b'\n');
                writer.set_color(Color::White, Color::Black);
            }
        }
    }

    fn cmd_pwd(&self, writer: &mut Writer) {
        let fs = get_filesystem();
        let mut path_buffer = [0u8; 128];
        let len = fs.get_current_path(&mut path_buffer);
        writer.set_color(Color::LightCyan, Color::Black);
        writer.write_bytes(&path_buffer[..len]);
        writer.write_byte(b'\n');
        writer.set_color(Color::White, Color::Black);
    }

    fn write_number(&self, writer: &mut Writer, mut num: usize) {
        if num == 0 {
            writer.write_byte(b'0');
            return;
        }
        
        let mut digits = [0u8; 20];
        let mut i = 0;
        while num > 0 {
            digits[i] = (num % 10) as u8 + b'0';
            num /= 10;
            i += 1;
        }
        
        while i > 0 {
            i -= 1;
            writer.write_byte(digits[i]);
        }
    }
}
