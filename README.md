# HexiumOS

A minimal 32-bit operating system kernel written in Rust, featuring a command-line interface, file system, text editor, Snake game, and ASCII video player.

![HexiumOS](https://img.shields.io/badge/language-Rust-orange.svg)
![Platform](https://img.shields.io/badge/platform-x86-blue.svg)
![License](https://img.shields.io/badge/license-MIT-green.svg)

## Features

### 🎮 Entertainment
- **Snake Game**: Classic snake game with keyboard controls
- **ASCII Video Player**: Play videos converted to ASCII art (includes Bad Apple demo)
- Real-time rendering with configurable FPS

### 💻 System Features
- **Command-Line Interface**: Interactive shell with multiple commands
- **File System**: In-memory file system with directory support
- **Text Editor**: Built-in editor for creating and modifying files
- **VGA Text Mode**: Custom VGA driver with color support
- **Keyboard Driver**: PS/2 keyboard input with interrupt handling
- **Interrupt Descriptor Table (IDT)**: Proper interrupt management

### 📁 File System Commands
- Create, read, write, and delete files
- Directory management (mkdir, rmdir, cd)
- File listing and navigation
- In-memory storage (up to 32 files, 4KB each)

## Prerequisites

- **Rust** (nightly toolchain)
- **NASM** or **GNU as** (assembler)
- **GNU ld** (linker)
- **QEMU** (for running the OS)
- **Python 3** with OpenCV (for video conversion)

## Building

### Quick Start

```bash
# Build the OS
make

# Build and run in QEMU
make run

# Clean build artifacts
make clean

# Rebuild from scratch
make rebuild
```

### Manual Build Steps

```bash
# Create bin directory
mkdir -p bin

# Assemble boot code
as --32 src/boot.asm -o bin/boot.o

# Compile Rust kernel
rustc --target i686-unknown-linux-gnu --crate-type staticlib \
      -C opt-level=2 -C panic=abort -C relocation-model=static \
      -C target-feature=-sse,-sse2,+soft-float \
      -o bin/kernel.o src/kernel.rs

# Link everything
ld -m elf_i386 -T src/linker.ld -o bin/myos.bin bin/boot.o bin/kernel.o

# Run in QEMU
qemu-system-i386 -kernel bin/myos.bin
```

### Build To ISO Steps

```bash
# Make iso
make iso

```

## Usage

### Available Commands

Once HexiumOS boots, you'll see the HexiumOS prompt. Type `help` to see available commands:

#### General Commands
- `help` - Display help information
- `clear` - Clear the screen
- `hello` - Print a greeting message
- `info` - Display system information
- `echo <text>` - Echo text back to the terminal

#### Entertainment
- `snake` - Launch the Snake game
  - Use arrow keys to control
  - ESC to exit
- `play <video>` - Play an ASCII video
  - `play badapple` - Play Bad Apple video
  - `play rahh` - Play RAHH video
  - ESC to exit playback

#### File System
- `ls` - List files and directories in current directory
- `cd <dir>` - Change to specified directory
- `pwd` - Print working directory
- `mkdir <dir>` - Create a new directory
- `rmdir <dir>` - Remove an empty directory
- `touch <file>` - Create an empty file
- `cat <file>` - Display file contents
- `edit <file>` - Open file in text editor
- `write <file>` - Write text to a file
- `rm <file>` - Delete a file

### Text Editor Controls
- Type to insert text
- `Backspace` - Delete character
- `ESC` - Save and exit editor

## Converting Videos to ASCII

HexiumOS includes a Python script to convert videos into ASCII art format:

```bash
python convert_video.py <video_file> [options]

Options:
  --width WIDTH      ASCII width (default: 80)
  --height HEIGHT    ASCII height (default: 24)
  --fps FPS          Target FPS (default: 15)
  --output OUTPUT    Output Rust file (default: src/bad_apple_data.rs)
  --invert           Invert colors (white on black)
```

### Example

```bash
# Convert a video to ASCII
python convert_video.py badapple.mp4 --width 80 --height 24 --fps 15 --output src/bad_apple_data.rs

# Rebuild the OS with the new video
make rebuild

# Run and play the video
make run
# Then type: play badapple
```
## Technical Details

### Architecture
- **Target**: i686 (32-bit x86)
- **Boot**: Custom bootloader using multiboot
- **Memory**: Direct VGA buffer access (0xB8000)
- **Interrupts**: Custom IDT with keyboard interrupt handler

### Rust Features Used
- `#![no_std]` - Bare metal development
- `#![no_main]` - Custom entry point
- Static compilation with panic=abort
- Soft-float arithmetic (no SSE/SSE2)

### Memory Layout
- **VGA Buffer**: 0xB8000 (80x25 text mode)
- **Kernel**: Loaded at 1MB physical address
- **File System**: Static arrays in kernel memory

### Video Player
- Stores pre-rendered ASCII frames
- Implements frame timing for consistent FPS
- Supports multiple video formats
- Direct VGA buffer manipulation for performance

### File System
- In-memory implementation
- Maximum 32 files
- Maximum 4KB per file
- Maximum 16 directories
- Hierarchical directory structure

## Controls

### General
- Type commands and press Enter
- Use Backspace to delete characters

### Snake Game
- `↑` Arrow Up - Move up
- `↓` Arrow Down - Move down
- `←` Arrow Left - Move left
- `→` Arrow Right - Move right
- `ESC` - Exit game

### Video Player
- `ESC` - Stop playback and return to CLI

### Text Editor
- Regular typing - Insert text
- `Backspace` - Delete character
- `ESC` - Save and exit

## Known Limitations

- No persistence (file system is RAM-only)
- Maximum file size: 4KB
- No multitasking
- No network support
- Limited to VGA text mode (80x25)
- No dynamic memory allocation

## Future Enhancements

- [ ] Persistent file system (disk I/O)
- [ ] More video codec support
- [ ] Additional games and applications
- [ ] Network stack
- [ ] GUI support
- [ ] Multi-processing
- [ ] Additional drivers (mouse, sound)

## Contributing

Contributions are welcome! Feel free to submit issues or pull requests.

## License

This project is open source and available under the MIT License.
