
use crate::idt;
use crate::pic;
use crate::keyboard;

#[unsafe(naked)]
unsafe extern "C" fn keyboard_interrupt_stub() {
    core::arch::naked_asm!(
        "push eax",
        "push ecx",
        "push edx",
        "push ebx",
        "push ebp",
        "push esi",
        "push edi",
        "call keyboard_handler_inner",
        "pop edi",
        "pop esi",
        "pop ebp",
        "pop ebx",
        "pop edx",
        "pop ecx",
        "pop eax",
        "iretd"
    );
}

#[no_mangle]
pub unsafe extern "C" fn default_handler_inner() {}


#[unsafe(naked)]
unsafe extern "C" fn default_interrupt_stub() {
    core::arch::naked_asm!(
        "push eax",
        "push ecx",
        "push edx",
        "push ebx",
        "push ebp",
        "push esi",
        "push edi",
        "call default_handler_inner",
        "pop edi",
        "pop esi",
        "pop ebp",
        "pop ebx",
        "pop edx",
        "pop ecx",
        "pop eax",
        "iretd"
    );
}

pub unsafe fn init() {
    pic::init();
    pic::disable_all();
    idt::init();
    
    for i in 0..32 {
        idt::set_handler(i, default_interrupt_stub);
    }
    
    idt::set_handler(33, keyboard_interrupt_stub);
    keyboard::init();
    pic::enable_irq(1);
    idt::enable_interrupts();
}
