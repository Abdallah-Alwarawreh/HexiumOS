const IDT_SIZE: usize = 256;

#[repr(C, packed)]
#[derive(Copy, Clone)]
pub struct IdtEntry {
    offset_low: u16,
    selector: u16,
    zero: u8,
    type_attr: u8,
    offset_high: u16,
}

impl IdtEntry {
    pub const fn empty() -> Self {
        IdtEntry {
            offset_low: 0,
            selector: 0,
            zero: 0,
            type_attr: 0,
            offset_high: 0,
        }
    }

    pub fn set_handler(&mut self, handler: unsafe extern "C" fn(), selector: u16, flags: u8) {
        let handler_addr = handler as u32;
        self.offset_low = handler_addr as u16;
        self.offset_high = (handler_addr >> 16) as u16;
        self.selector = selector;
        self.zero = 0;
        self.type_attr = flags;
    }
}

#[repr(C, packed)]
pub struct IdtPtr {
    limit: u16,
    base: u32,
}

static mut IDT: [IdtEntry; IDT_SIZE] = [IdtEntry::empty(); IDT_SIZE];
static mut IDT_PTR: IdtPtr = IdtPtr { limit: 0, base: 0 };

pub const IDT_PRESENT: u8 = 0x80;
pub const IDT_INTERRUPT_GATE: u8 = 0x0E;
pub const IDT_DPL_RING0: u8 = 0x00;

pub unsafe fn init() {
    IDT_PTR.limit = (core::mem::size_of::<[IdtEntry; IDT_SIZE]>() - 1) as u16;
    IDT_PTR.base = IDT.as_ptr() as u32;

    core::arch::asm!(
        "lidt [{}]",
        in(reg) &IDT_PTR
    );
}

pub unsafe fn set_handler(index: usize, handler: unsafe extern "C" fn()) {
    IDT[index].set_handler(
        handler,
        0x08,
        IDT_PRESENT | IDT_INTERRUPT_GATE | IDT_DPL_RING0,
    );
}

pub unsafe fn enable_interrupts() {
    core::arch::asm!("sti");
}

pub unsafe fn disable_interrupts() {
    core::arch::asm!("cli");
}
