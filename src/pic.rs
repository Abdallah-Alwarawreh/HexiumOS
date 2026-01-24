use crate::io::{inb, outb, io_wait};

const PIC1_COMMAND: u16 = 0x20;
const PIC1_DATA: u16 = 0x21;
const PIC2_COMMAND: u16 = 0xA0;
const PIC2_DATA: u16 = 0xA1;

const ICW1_INIT: u8 = 0x10;
const ICW1_ICW4: u8 = 0x01;
const ICW4_8086: u8 = 0x01;

const PIC_EOI: u8 = 0x20;

pub unsafe fn init() {
    let mask1 = inb(PIC1_DATA);
    let mask2 = inb(PIC2_DATA);

    outb(PIC1_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();
    outb(PIC2_COMMAND, ICW1_INIT | ICW1_ICW4);
    io_wait();

    outb(PIC1_DATA, 0x20);
    io_wait();
    outb(PIC2_DATA, 0x28);
    io_wait();

    outb(PIC1_DATA, 4);
    io_wait();
    outb(PIC2_DATA, 2);
    io_wait();

    outb(PIC1_DATA, ICW4_8086);
    io_wait();
    outb(PIC2_DATA, ICW4_8086);
    io_wait();

    outb(PIC1_DATA, mask1);
    outb(PIC2_DATA, mask2);
}

pub unsafe fn enable_irq(irq: u8) {
    let port: u16;
    let irq_line: u8;

    if irq < 8 {
        port = PIC1_DATA;
        irq_line = irq;
    } else {
        port = PIC2_DATA;
        irq_line = irq - 8;
    }

    let mask = inb(port) & !(1 << irq_line);
    outb(port, mask);
}

pub unsafe fn disable_irq(irq: u8) {
    let port: u16;
    let irq_line: u8;

    if irq < 8 {
        port = PIC1_DATA;
        irq_line = irq;
    } else {
        port = PIC2_DATA;
        irq_line = irq - 8;
    }

    let mask = inb(port) | (1 << irq_line);
    outb(port, mask);
}

pub unsafe fn send_eoi(irq: u8) {
    if irq >= 8 {
        outb(PIC2_COMMAND, PIC_EOI);
    }
    outb(PIC1_COMMAND, PIC_EOI);
}

pub unsafe fn disable_all() {
    outb(PIC1_DATA, 0xFF);
    outb(PIC2_DATA, 0xFF);
}
