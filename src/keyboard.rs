use crate::io::inb;
use crate::pic;

const KEYBOARD_DATA_PORT: u16 = 0x60;
const KEYBOARD_STATUS_PORT: u16 = 0x64;

const BUFFER_SIZE: usize = 256;
static mut KEY_BUFFER: [u8; BUFFER_SIZE] = [0; BUFFER_SIZE];
static mut BUFFER_HEAD: usize = 0;
static mut BUFFER_TAIL: usize = 0;

static mut SHIFT_PRESSED: bool = false;
static mut CTRL_PRESSED: bool = false;
static mut ALT_PRESSED: bool = false;
static mut CAPS_LOCK: bool = false;

const SCANCODE_TO_ASCII: [u8; 128] = [
    0, 27, b'1', b'2', b'3', b'4', b'5', b'6', b'7', b'8',
    b'9', b'0', b'-', b'=', 8, b'\t',
    b'q', b'w', b'e', b'r', b't', b'y', b'u', b'i', b'o', b'p',
    b'[', b']', b'\n', 0,
    b'a', b's', b'd', b'f', b'g', b'h', b'j', b'k', b'l', b';',
    b'\'', b'`', 0, b'\\',
    b'z', b'x', b'c', b'v', b'b', b'n', b'm', b',', b'.', b'/',
    0, b'*', 0, b' ', 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
    b'7', b'8', b'9', b'-',
    b'4', b'5', b'6', b'+',
    b'1', b'2', b'3',
    b'0', b'.',
    0, 0, 0,
    0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];

const SCANCODE_TO_ASCII_SHIFT: [u8; 128] = [
    0, 27, b'!', b'@', b'#', b'$', b'%', b'^', b'&', b'*',
    b'(', b')', b'_', b'+', 8, b'\t',
    b'Q', b'W', b'E', b'R', b'T', b'Y', b'U', b'I', b'O', b'P',
    b'{', b'}', b'\n', 0,
    b'A', b'S', b'D', b'F', b'G', b'H', b'J', b'K', b'L', b':',
    b'"', b'~', 0, b'|',
    b'Z', b'X', b'C', b'V', b'B', b'N', b'M', b'<', b'>', b'?',
    0, b'*', 0, b' ', 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0,
    b'7', b'8', b'9', b'-',
    b'4', b'5', b'6', b'+',
    b'1', b'2', b'3',
    b'0', b'.',
    0, 0, 0,
    0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 0, 0, 0, 0, 0, 0, 0, 0,
];

pub const KEY_BACKSPACE: u8 = 8;
pub const KEY_TAB: u8 = 9;
pub const KEY_ENTER: u8 = 10;
pub const KEY_ESCAPE: u8 = 27;

unsafe fn read_scancode() -> u8 {
    inb(KEYBOARD_DATA_PORT)
}

unsafe fn process_scancode(scancode: u8) {
    let is_release = scancode & 0x80 != 0;
    let key = scancode & 0x7F;

    match key {
        0x2A | 0x36 => {
            SHIFT_PRESSED = !is_release;
        }
        0x1D => {
            CTRL_PRESSED = !is_release;
        }
        0x38 => {
            ALT_PRESSED = !is_release;
        }
        0x3A => {
            if !is_release {
                CAPS_LOCK = !CAPS_LOCK;
            }
        }
        _ => {
            if !is_release {
                let ascii = scancode_to_ascii(key);
                if ascii != 0 {
                    push_key(ascii);
                }
            }
        }
    }
}

unsafe fn scancode_to_ascii(scancode: u8) -> u8 {
    if scancode as usize >= SCANCODE_TO_ASCII.len() {
        return 0;
    }

    let mut ascii = if SHIFT_PRESSED {
        SCANCODE_TO_ASCII_SHIFT[scancode as usize]
    } else {
        SCANCODE_TO_ASCII[scancode as usize]
    };

    if CAPS_LOCK && ascii >= b'a' && ascii <= b'z' {
        ascii -= 32;
    } else if CAPS_LOCK && ascii >= b'A' && ascii <= b'Z' && !SHIFT_PRESSED {
    } else if CAPS_LOCK && SHIFT_PRESSED && ascii >= b'A' && ascii <= b'Z' {
        ascii += 32;
    }

    ascii
}

unsafe fn push_key(key: u8) {
    let next_head = (BUFFER_HEAD + 1) % BUFFER_SIZE;
    if next_head != BUFFER_TAIL {
        KEY_BUFFER[BUFFER_HEAD] = key;
        BUFFER_HEAD = next_head;
    }
}

pub fn pop_key() -> Option<u8> {
    unsafe {
        if BUFFER_HEAD == BUFFER_TAIL {
            None
        } else {
            let key = KEY_BUFFER[BUFFER_TAIL];
            BUFFER_TAIL = (BUFFER_TAIL + 1) % BUFFER_SIZE;
            Some(key)
        }
    }
}

pub fn key_available() -> bool {
    unsafe { BUFFER_HEAD != BUFFER_TAIL }
}

pub fn get_modifiers() -> (bool, bool, bool, bool) {
    unsafe { (SHIFT_PRESSED, CTRL_PRESSED, ALT_PRESSED, CAPS_LOCK) }
}

#[no_mangle]
pub unsafe extern "C" fn keyboard_handler_inner() {
    let scancode = read_scancode();
    process_scancode(scancode);
    pic::send_eoi(1);
}

pub unsafe fn init() {
    while inb(KEYBOARD_STATUS_PORT) & 1 != 0 {
        inb(KEYBOARD_DATA_PORT);
    }
}
